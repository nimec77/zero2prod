use wiremock::{
    Mock, ResponseTemplate,
    matchers::{any, method, path},
};

use crate::helpers::{TestApp, fake_email, fake_name, spawn_app};

#[tokio::test]
async fn newsletters_are_not_delivered_to_unconfirmed_subscribers() {
    let test_app = spawn_app().await;
    create_unconfirmed_subscriber(&test_app).await;

    let _ = Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount_as_scoped(&test_app.email_server)
        .await;

    // Act
    // A sketch of the newsletter payload structure.
    // We might change it later on.
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "content": {
        "text": "Newsletter body as plain text",
        "html": "<p>Newsletter body as HTML</p>",
        }
    });

    let response = reqwest::Client::new()
        .post(format!("{}/newsletter", &test_app.address))
        .json(&newsletter_request_body)
        .send()
        .await
        .expect("Failed to send newsletter request.");

    // Assert
    assert_eq!(response.status().as_u16(), 200);
}

async fn create_unconfirmed_subscriber(test_app: &TestApp) {
    let name = fake_name();
    let email = fake_email().as_ref().to_owned();
    let body = format!("name={name}&email={email}");

    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .named("Create unconfirmed subscriber")
        .expect(1)
        .mount_as_scoped(&test_app.email_server)
        .await;

    test_app.post_subscriptions(body).await;
}
