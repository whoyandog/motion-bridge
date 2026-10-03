use crate::mocap;
use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::get,
};
use futures::stream::StreamExt;
use prost::Message as ProstMessage;

pub fn create_router() -> Router {
    Router::new().route("/ws", get(ws_handler))
}

async fn ws_handler(ws: WebSocketUpgrade) -> axum::response::Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    println!("Клиент подключен по WS (TCP)");

    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            Message::Binary(bytes) => match mocap::MocapFrame::decode(bytes) {
                Ok(frame) => {
                    println!(
                        "Тип: {}, точек: {}, время: {}",
                        frame.skeleton_type,
                        frame.landmarks.len(),
                        frame.timestamp
                    );
                }
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
