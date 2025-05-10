use crate::helpers::{spawn_app, UrlEncodable};

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let test_app = spawn_app().await;

    // Act
    let name = "le guin";
    let email = "ursula_le_guin@gmail.com";
    let body = format!("name={}&email={}", name.url_encode(), email.url_encode());
    // let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = test_app.post_subscriptions(body).await;

    let saved = sqlx::query!(
        r#"
        SELECT email, 
            name
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
    assert_eq!(200, response.status().as_u16());
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let test_app = spawn_app().await;
    let name = "le guin";
    let email = "ursula_le_guin@gmail.co";
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
    let name = "Ursula";
    let email = "ursula_le_guin2@gmail.com";
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
