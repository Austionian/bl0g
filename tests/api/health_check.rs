use crate::helpers::start_test_app;

#[tokio::test]
async fn it_returns_the_index() {
    let addr = start_test_app()
        .await
        .expect("Unable to start test server.");

    let response = reqwest::get(format!("http://{}/health_check", &addr))
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 200);
}
