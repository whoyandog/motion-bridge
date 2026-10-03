pub mod mocap {
    include!(concat!(env!("OUT_DIR"), "/mocap.rs"));
}

mod network;
mod solver;

use tokio::sync::watch;

#[tokio::main]
async fn main() {
    let (tx_raw, rx_raw) = watch::channel(None);
    let (tx_solved, rx_solved) = watch::channel(None);

    tokio::spawn(async move {
        network::udp_client::start(rx_solved).await;
    });

    tokio::spawn(async move {
        solver::run(rx_raw, tx_solved).await;
    });

    let app = network::ws_server::create_router(tx_raw);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Ядро запущено на {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
