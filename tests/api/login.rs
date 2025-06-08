use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    // Arrange
    let test_app = spawn_app().await;

    // Act
    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password"
    });
    let response = test_app.post_login(&login_body).await;

    // Assert
    assert_is_redirect_to(&response, "/login");

    // Act - Part 2
    let html_page = test_app.get_login_html().await;
    assert!(html_page.contains("<p><i>Authentication failed</i></p>"));

    // Act - Part 3 - Reload the login page
    let html_page = test_app.get_login_html().await;
    assert!(!html_page.contains("<p><i>Authentication failed</i></p>"));
}

#[tokio::test]
async fn redirect_to_admin_dashboard_after_login_success() {
    // Arrange
    let test_app = spawn_app().await;

    // Act - Part 1 - Login
    let login_body = serde_json::json!({
    "username": &test_app.test_user.username,
    "password": &test_app.test_user.password
    });
    let response = test_app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    // Act - Part 2 - Follow the redirect
    let html_page = test_app.get_admin_dashboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}", test_app.test_user.username)));
}
