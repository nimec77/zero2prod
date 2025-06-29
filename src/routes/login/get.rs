use actix_web::{HttpResponse, http::header::ContentType};
use actix_web_flash_messages::IncomingFlashMessages;

pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut error_html = String::new();
    for m in flash_messages.iter() {
        error_html.push_str(&format!("<p><i>{}</i></p>", m.content()));
    }
    let html = format!(
        r#"<!DOCTYPE html>
            <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Login</title>
            </head>
            <body>
                {error_html}
                <form action="/login" method="post">
                    <label>Username
                        <input type="text" placeholder="Enter Username" name="username">
                    </label><label>Password
                        <input type="password" placeholder="Enter Password" name="password">
                    </label>
                    <button type="submit">Login</button>
                </form>
            </body>
            </html>"#
    );

    HttpResponse::Ok()
        // No more removal cookie!
        .content_type(ContentType::html())
        .body(html)
}
