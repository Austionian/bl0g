use crate::helpers::start_test_app;
use hyper::{Body, Request};

#[tokio::test]
async fn it_returns_the_index() {
    let addr = start_test_app()
        .await
        .expect("Unable to start test server.");

    let client = hyper::Client::new();

    let response = client
        .request(
            Request::builder()
                .uri(format!("http://{}", &addr))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 200);
}
