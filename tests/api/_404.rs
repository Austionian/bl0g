use crate::helpers::start_test_app;
use hyper::{Body, Request};

#[tokio::test]
async fn it_returns_the_404_page() {
    let addr = start_test_app()
        .await
        .expect("Unable to start test server.");

    let client = hyper::Client::new();

    let (parts, body) = client
        .request(
            Request::builder()
                .uri(format!("http://{}/some_unknown_path", &addr))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
        .into_parts();

    assert_eq!(parts.status.as_u16(), 404);

    let body = hyper::body::to_bytes(body).await.unwrap();
    let body = String::from_utf8(body.to_vec()).unwrap();

    assert!(body.contains("nothing to see here."));
}
