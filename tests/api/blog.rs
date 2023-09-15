use crate::helpers::start_test_app;
use hyper::{Body, Request};

#[tokio::test]
async fn it_returns_the_blog_page_with_full_template() {
    let addr = start_test_app()
        .await
        .expect("Unable to start test server.");

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
