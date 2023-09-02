use bl0g::startup;
use hyper::{Body, Request};
use std::net::{SocketAddr, TcpListener};

// TODO: Write a test like this one that isn't relying on the
// local filesystem.
#[tokio::test]
async fn it_return_the_full_post_template() {
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
                .uri(format!("http://{}/bl0g/hello_world", &addr))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
        .into_parts();

    assert_eq!(parts.status.as_u16(), 200);

    let body = hyper::body::to_bytes(body).await.unwrap();
    let body = String::from_utf8(body.to_vec()).unwrap();

    assert!(body.contains("Hello!"));

    // Text from the header
    assert!(body.contains("ab0ut"));
    // Text from the footer
    assert!(body.contains("Austin Rooks. All rights reserved."));
}

#[tokio::test]
async fn it_return_the_just_the_post_text() {
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
                .uri(format!("http://{}/bl0g/hello_world", &addr))
                .header("HX-Request", "true")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
        .into_parts();

    assert_eq!(parts.status.as_u16(), 200);

    let body = hyper::body::to_bytes(body).await.unwrap();
    let body = String::from_utf8(body.to_vec()).unwrap();

    assert!(body.contains("Hello!"));

    // Text from the header
    assert!(!body.contains("ab0ut"));
    // Text from the footer
    assert!(!body.contains("Austin Rooks. All rights reserved."));
}
