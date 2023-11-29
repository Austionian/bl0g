use crate::helpers::start_test_app;
use hyper::{Body, Request};

// TODO: Write a test like this one that isn't relying on the
// local filesystem.
#[tokio::test]
async fn it_return_the_full_post_template() {
    let addr = start_test_app()
        .await
        .expect("Unable to start test server.");

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
    assert!(body.contains("bl0g"));
    // Text from the footer
    assert!(body.contains("Austin Rooks. All rights reserved."));
}

#[tokio::test]
async fn it_return_the_just_the_post_text() {
    let addr = start_test_app()
        .await
        .expect("Unable to start test server.");

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
    assert!(!body.contains("bl0g"));
    // Text from the footer
    assert!(!body.contains("Austin Rooks. All rights reserved."));
}
