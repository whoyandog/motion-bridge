use crate::motion_bridge_schema;
use crate::motion_bridge_schema::capture_event_packet::Payload;
use crate::solver::types::SolverEvent;
use axum::{
    Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    routing::get,
};
use futures::stream::StreamExt;
use prost::Message as ProstMessage;
use tokio::sync::mpsc;

type SharedState = mpsc::Sender<SolverEvent>;

pub fn create_router(tx: mpsc::Sender<SolverEvent>) -> Router {
    Router::new().route("/ws", get(ws_handler)).with_state(tx)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> axum::response::Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, tx: SharedState) {
    println!("[WS] Клиент подключен по WS (TCP)");

    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            Message::Binary(bytes) => {
                match motion_bridge_schema::CaptureEventPacket::decode(bytes) {
                    Ok(event) => match event.payload {
                        Some(Payload::Handshake(handshake)) => {
                            println!("[WS] Handshake: {}", handshake.model_name);
                            let _ = tx
                                .send(SolverEvent::Handshake {
                                    model_name: handshake.model_name,
                                })
                                .await;
                        }
                        Some(Payload::Body(body)) => {
                            let _ = tx.try_send(SolverEvent::BodyFrame {
                                timestamp: event.timestamp,
                                actor_id: event.actor_id,
                                landmarks_data: body.landmarks_data,
                            });
                        }
                        None => {
                            eprintln!("[WS] Пустое сообщение или нет payload");
                        }
                    },
                    Err(e) => {
                        eprintln!("[WS] Ошибка парсига protobuf: {}", e);
                    }
                }
            }
            Message::Close(_) => {
                println!("[WS] Клиент отключен");
                break;
            }
            _ => {}
        }
    }
}
