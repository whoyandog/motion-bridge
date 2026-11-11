use crate::motion_bridge_schema::RigUpdatePacket;
use prost::Message;
use tokio::net::UdpSocket;
use tokio::sync::watch;

pub async fn start(mut rx_solved: watch::Receiver<Option<RigUpdatePacket>>) {
    let socket = UdpSocket::bind("127.0.0.1:0")
        .await
        .expect("Не удалось создать UDP сокет");
    let target_addr = "127.0.0.1:4242";

    let mut buf = Vec::with_capacity(1500);

    loop {
        if rx_solved.changed().await.is_err() {
            println!("[UDP] Канал закрыт.");
            break;
        }

        let frame_opt = rx_solved.borrow_and_update().clone();

        if let Some(frame) = frame_opt {
            buf.clear();
            if frame.encode(&mut buf).is_ok() {
                if let Err(e) = socket.send_to(&buf, target_addr).await {
                    eprintln!("[UDP] Ошибка отправки пакета: {}", e);
                }
            }
        }
    }
}
