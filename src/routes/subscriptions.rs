use serde::Deserialize;
use actix_web::{web, HttpResponse, Responder};
#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscriptions(form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
