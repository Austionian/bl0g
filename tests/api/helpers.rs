use bl0g::startup;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn start_test_app() -> Result<SocketAddr, String> {
    let app = startup().expect("Unable to start the server.");
    let listener = TcpListener::bind("127.0.0.1:0".parse::<SocketAddr>().unwrap())
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();

    let _ = tokio::spawn(async move {
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    });

    Ok(addr)
}
