use crate::helpers::start_test_app;

// TODO: Write a test like this one that isn't relying on the
// local filesystem.
#[tokio::test]
async fn it_return_the_full_post_template() {
    let addr = start_test_app()
        .await
        .expect("Unable to start test server.");

    let response = reqwest::get(format!("http://{}/bl0g/hello_world", &addr))
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 200);

    let body = response.text().await.unwrap();

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

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/bl0g/hello_world", &addr))
        .header("HX-Request", "true")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 200);

    let body = response.text().await.unwrap();

    assert!(body.contains("Hello!"));

    // Text from the header
    assert!(!body.contains("bl0g"));
    // Text from the footer
    assert!(!body.contains("Austin Rooks. All rights reserved."));
}
