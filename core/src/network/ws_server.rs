use crate::motion_bridge_schema;
use crate::motion_bridge_schema::capture_event_packet::Payload;
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
use std::sync::Arc;
use tokio::sync::watch;

type SharedState = Arc<watch::Sender<Option<motion_bridge_schema::CaptureEventPacket>>>;

pub fn create_router(
    tx: watch::Sender<Option<motion_bridge_schema::CaptureEventPacket>>,
) -> Router {
    let state = Arc::new(tx);
    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> axum::response::Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, tx: SharedState) {
    println!("Клиент подключен по WS (TCP)");

    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            Message::Binary(bytes) => match motion_bridge_schema::CaptureEventPacket::decode(bytes)
            {
                Ok(event) => match event.payload {
                    Some(Payload::Handshake(handshake)) => {
                        println!("Получен handshake, тип скелета: {}", handshake.model_name);
                    }
                    Some(Payload::Body(ref _body)) => {
                        let _ = tx.send(Some(event));
                    }
                    None => {
                        eprintln!("Получено пустое сообщение (payload отсутствует)");
                    }
                },
                Err(e) => {
                    eprintln!("Ошибка парсига protobuf: {}", e);
                }
            },
            Message::Close(_) => {
                println!("Клиент отключен");
                break;
            }
            _ => {}
        }
    }
}
