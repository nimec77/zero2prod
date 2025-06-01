use wiremock::{
    Mock, ResponseTemplate,
    matchers::{method, path},
};

use crate::helpers::{UrlEncodable, fake_email, fake_name, spawn_app};

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let test_app = spawn_app().await;

    let name = fake_name();
    let email = fake_email().as_ref().to_owned();
    let body = format!("name={}&email={}", name.url_encode(), email.url_encode());

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&test_app.email_server)
        .await;

    // Act
    let response = test_app.post_subscriptions(body).await;

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_persists_the_new_subscriber() {
    // Arrange
    let test_app = spawn_app().await;
    let name = fake_name();
    let email = fake_email().as_ref().to_owned();
    let body = format!("name={}&email={}", name.url_encode(), email.url_encode());

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&test_app.email_server)
        .await;

    test_app.post_subscriptions(body).await;

    let saved = sqlx::query!(
        r#"
        SELECT email, 
            name,
            status
        FROM subscriptions 
        WHERE email = $1 
        AND name = $2"#,
        email,
        name
    )
    .fetch_one(&test_app.db_pool)
    .await
    .expect("Failed to fetch saved subscriptions.");

    // Assert
    assert_eq!(saved.email, email);
    assert_eq!(saved.name, name);
    assert_eq!(saved.status, "pending_confirmation");
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let test_app = spawn_app().await;
    let name = fake_name();
    let email = fake_email().as_ref().to_owned();
    let test_cases = vec![
        (format!("name={}", name.url_encode()), "missing the email"),
        (format!("email={}", email.url_encode()), "missing the name"),
        ("".to_owned(), "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = test_app.post_subscriptions(invalid_body).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {error_message}"
        );
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    // Arrange
    let app = spawn_app().await;
    let name = fake_name();
    let email = fake_email().as_ref().to_owned();
    let test_cases = vec![
        (format!("name=&email={}", email.url_encode()), "empty name"),
        (format!("name={}&email=", name.url_encode()), "empty email"),
        (
            format!("name={}&email=definitely-not-an-email", name.url_encode()),
            "invalid email",
        ),
    ];
    for (body, description) in test_cases {
        // Act
        let response = app.post_subscriptions(body).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {description}."
        );
    }
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_for_valid_data() {
    // Arrange
    let test_app = spawn_app().await;
    let email = fake_email().as_ref().to_owned();
    let name = fake_name();
    let body = format!("name={}&email={}", name.url_encode(), email.url_encode());

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&test_app.email_server)
        .await;

    // Act
    test_app.post_subscriptions(body).await;

    // Assert
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_with_a_link() {
    // Arrange
    let test_app = spawn_app().await;
    let email = fake_email().as_ref().to_owned();
    let name = fake_name();
    let body = format!("name={}&email={}", name.url_encode(), email.url_encode());

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&test_app.email_server)
        .await;

    // Act
    test_app.post_subscriptions(body).await;

    // Assert
    let email_request = &test_app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = test_app.get_confirmation_links(email_request);

    assert_eq!(confirmation_links.html, confirmation_links.plain_text);
}

// This ignore don't remove, it's a test for the database error
#[tokio::test]
#[ignore]
async fn subscribe_fails_if_there_is_a_fatal_database_error() {
    // Arrange
    let test_app = spawn_app().await;
    let email = fake_email().as_ref().to_owned();
    let name = fake_name();
    let body = format!("name={}&email={}", name.url_encode(), email.url_encode());

    // Sabotage the database
    sqlx::query!("ALTER TABLE subscription_tokens DROP COLUMN subscription_token;",)
        .execute(&test_app.db_pool)
        .await
        .unwrap();
    // Act
    let response = test_app.post_subscriptions(body).await;

    // Assert
    assert_eq!(response.status().as_u16(), 500);
}
