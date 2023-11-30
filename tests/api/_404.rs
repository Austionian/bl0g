use crate::helpers::start_test_app;

#[tokio::test]
async fn it_returns_the_404_page() {
    let addr = start_test_app()
        .await
        .expect("Unable to start test server.");

    let response = reqwest::get(format!("http://{}/some_unknown_path", &addr))
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 404);

    let body = response.text().await.unwrap();

    assert!(body.contains("nothing to see here."));
}
