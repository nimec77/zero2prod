use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use sqlx::{
    PgPool,
    types::chrono::Utc,
};
#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscriptions(
    form: web::Form<FormData>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
