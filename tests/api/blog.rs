use bl0g::startup;
use hyper::{Body, Request};
use std::net::{SocketAddr, TcpListener};

#[tokio::test]
async fn it_returns_the_blog_page_with_full_template() {
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

    let (parts, body) = client
        .request(
            Request::builder()
                .uri(format!("http://{}/bl0g", &addr))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
        .into_parts();

    assert_eq!(parts.status.as_u16(), 200);

    let body = hyper::body::to_bytes(body).await.unwrap();
    let body = String::from_utf8(body.to_vec()).unwrap();

    assert!(body.contains("selected writings"));
    // Text from the header.
    assert!(body.contains("_"));
    // Text from the footer.
    assert!(body.contains("ab0ut"));
}
