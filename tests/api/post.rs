use bl0g::startup;
use hyper::{Body, Request};
use std::net::{SocketAddr, TcpListener};

// TODO: Write a test like this one that isn't relying on the
// local filesystem.
#[tokio::test]
async fn it_returns_the_index() {
    let app = startup();
    let listener = TcpListener::bind("127.0.0.1:0".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listener.local_addr().unwrap();

    let _ = tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    let client = hyper::Client::new();

    let response = client
        .request(
            Request::builder()
                .uri(format!("http://{}/bl0g/hello_world", &addr))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 200);

    // let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    // assert_eq!(&body[..], b"jkjk");
}
