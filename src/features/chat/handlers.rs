use std::time::{Duration, Instant};

use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use sqlx::PgPool;

use crate::features::chat::{
    dto::{GetMessagesQuery, SendMessageRequest},
    service::{ChatService, ChatServiceError},
    ws_events::{ClientEvent, ServerEvent},
};

pub async fn send_message(
    pool: web::Data<PgPool>,
    code: web::Path<String>,
    body: web::Json<SendMessageRequest>,
) -> impl Responder {
    match ChatService::send_message(&pool, &code.into_inner(), body.participant_id, &body.content).await {
        Ok(message) => HttpResponse::Created().json(message),
        Err(ChatServiceError::RoomNotFound) => HttpResponse::NotFound().body("Room not found"),
        Err(ChatServiceError::ParticipantNotInRoom) => {
            HttpResponse::BadRequest().body("Participant is not active in this room")
        }
        Err(ChatServiceError::InvalidMessageContent) => {
            HttpResponse::BadRequest().body("Message content must be 1..2000 characters")
        }
        Err(ChatServiceError::DatabaseError(err)) => {
            HttpResponse::InternalServerError().body(format!("Database error: {}", err))
        }
    }
}

pub async fn get_messages(
    pool: web::Data<PgPool>,
    code: web::Path<String>,
    query: web::Query<GetMessagesQuery>,
) -> impl Responder {
    match ChatService::get_messages(&pool, &code.into_inner(), &query).await {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(ChatServiceError::RoomNotFound) => HttpResponse::NotFound().body("Room not found"),
        Err(ChatServiceError::DatabaseError(err)) => {
            HttpResponse::InternalServerError().body(format!("Database error: {}", err))
        }
        Err(ChatServiceError::InvalidMessageContent) => {
            HttpResponse::BadRequest().body("Invalid message content")
        }
        Err(ChatServiceError::ParticipantNotInRoom) => {
            HttpResponse::BadRequest().body("Participant is not active in this room")
        }
    }
}

pub async fn ws_connect(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let room_code = path.into_inner();
    let session = ChatWsSession::new(room_code);
    ws::start(session, &req, stream)
}

struct ChatWsSession {
    room_code: String,
    heartbeat: Instant,
}

impl ChatWsSession {
    fn new(room_code: String) -> Self {
        Self {
            room_code,
            heartbeat: Instant::now(),
        }
    }

    fn send_event(ctx: &mut ws::WebsocketContext<Self>, event: &ServerEvent) {
        match serde_json::to_string(event) {
            Ok(text) => ctx.text(text),
            Err(_) => ctx.stop(),
        }
    }
}

impl Actor for ChatWsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            if Instant::now().duration_since(act.heartbeat) > Duration::from_secs(15) {
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });

        Self::send_event(
            ctx,
            &ServerEvent::Joined {
                participant_id: uuid::Uuid::nil(),
                nickname: "connected".to_string(),
            },
        );
    }
}


//main socket handler broadcasting isnt implemented yet and persistence is not wired
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatWsSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(bytes)) => {
                self.heartbeat = Instant::now();
                ctx.pong(&bytes);
            }
            Ok(ws::Message::Pong(_)) => {
                self.heartbeat = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let parsed = serde_json::from_str::<ClientEvent>(&text);

                match parsed {
                    Ok(ClientEvent::Ping) => {
                        Self::send_event(ctx, &ServerEvent::Pong);
                    }
                    Ok(ClientEvent::Join { participant_id, nickname }) => {
                        Self::send_event(
                            ctx,
                            &ServerEvent::Joined {
                                participant_id,
                                nickname,
                            },
                        );
                    }
                    Ok(ClientEvent::Message { participant_id, content }) => {
                        let event = ServerEvent::Error {
                            message: format!(
                                "Message received in room {} from {} with {} chars. Persistence and broadcasting happen next.",
                                self.room_code,
                                participant_id,
                                content.len(),
                            ),
                        };
                        Self::send_event(ctx, &event);
                    }
                    Ok(ClientEvent::Leave { participant_id: _ }) => {
                        ctx.close(None);
                        ctx.stop();
                    }
                    Err(_) => {
                        Self::send_event(
                            ctx,
                            &ServerEvent::Error {
                                message: "invalid websocket event payload".to_string(),
                            },
                        );
                    }
                }
            }
            Ok(ws::Message::Close(reason)) => {
                if let Some(reason) = reason {
                    tracing::debug!(?reason, "WebSocket closed");
                }
                ctx.stop();
            }
            Ok(ws::Message::Binary(_)) => {}
            Ok(ws::Message::Continuation(_)) => {}
            Ok(ws::Message::Nop) => {}
            Err(_) => {
                ctx.stop();
            }
        }
    }
}
