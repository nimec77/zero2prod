use actix_web::{HttpResponse, http::header::ContentType};
use actix_web_flash_messages::IncomingFlashMessages;
use uuid::Uuid;

use crate::{
    session_state::TypedSession,
    utils::{e500, see_other},
};

pub async fn publish_newsletter_form(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(see_other("/login"));
    }
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        msg_html.push_str(&format!("<p><i>{}</i></p>\n", m.content()));
    }
    let key = Uuid::new_v4();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
        <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Publish Newsletter</title>
            </head>
            <body>
                {msg_html}
                <form action="/admin/newsletter" method="post">
                    <label>Title
                        <input type="text" name="title" placeholder="Newsletter title">
                    </label>
                    <br>
                    <label>Text content
                        <textarea name="text_content" placeholder="Text version of the newsletter"></textarea>
                    </label>
                    <br>
                    <label>HTML content
                        <textarea name="html_content" placeholder="HTML version of the newsletter"></textarea>
                    </label>
                    <br>
                    <input type="hidden" name="idempotency_key" value="{key}">
                    <button type="submit">Send newsletter</button>
                </form>
                <p><a href="/admin/dashboard">&lt;- Back</a></p>
            </body>
        </html>
        "#,
        )))
}
