use helpers::spawn_app;

mod helpers;

#[tokio::test(flavor = "multi_thread")]
async fn health_check_works() {
    // Setup
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Test
    let response = client
        .get(format!("{}/health_check", test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
