use crate::helpers::start_test_app;

#[tokio::test]
async fn it_returns_the_blog_page_with_full_template() {
    let addr = start_test_app()
        .await
        .expect("Unable to start test server.");

    let response = reqwest::get(format!("http://{}/bl0g", &addr))
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 200);

    let body = response.text().await.unwrap();

    assert!(body.contains("bl0g"));
    // Text from the header.
    assert!(body.contains("pr0jects"));
    // Text from the footer.
    assert!(body.contains("©"));
}
