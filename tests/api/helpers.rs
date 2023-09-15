use bl0g::startup;
use std::net::{SocketAddr, TcpListener};

pub async fn start_test_app() -> Result<SocketAddr, String> {
    let app = startup().expect("Unable to start the server.");
    let listener = TcpListener::bind("127.0.0.1:0".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listener.local_addr().unwrap();

    let _ = tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    Ok(addr)
}
