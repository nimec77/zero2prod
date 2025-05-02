use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::{PgPool, types::chrono::Utc};
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),    
    fields(
    subscriber_email = %form.email,
    subscriber_name= %form.name
    )
)]
pub async fn subscriptions(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    if !is_valid_name(&form.name) {
        return HttpResponse::BadRequest().finish();
    }
    match insert_subscriber(&form, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(form: &FormData, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {e:?}");
        e
    })?;

    Ok(())
}

pub fn is_valid_name(name: &str) -> bool {
    let its_empty_or_whitespace = name.trim().is_empty();
    let its_too_long = name.graphemes(true).count() > 256;
    let forbidden_characters = ['/', '(', ')','"', '<', '>','\\', '{', '}'];
    let contains_forbidden_characters = name.chars().any(|c| forbidden_characters.contains(&c));

    !(its_empty_or_whitespace || its_too_long || contains_forbidden_characters)
}

