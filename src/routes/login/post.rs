use actix_web::{
    HttpResponse, ResponseError,
    error::InternalError,
    http::{
        StatusCode,
        header::{self},
    },
    web,
};
use actix_web_flash_messages::FlashMessage;
use hmac::{Hmac, Mac};
use secrecy::SecretString;
use sqlx::PgPool;

use crate::{
    authentication::{AuthError, Credentials, validate_credentials},
    routes::error_chain_fmt,
    session_state::TypedSession,
};

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: SecretString,
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for LoginError {
    fn error_response(&self) -> HttpResponse {
        let query_string = format!("error={}", urlencoding::Encoded::new(self.to_string()));
        let secret: &[u8] = &[];
        let hmac_tag = {
            let mut mac = Hmac::<sha2::Sha256>::new_from_slice(secret).unwrap();
            mac.update(query_string.as_bytes());

            mac.finalize().into_bytes()
        };
        // let encoded_error = urlencoding::Encoded::new(self.to_string());
        HttpResponse::build(self.status_code())
            .insert_header((
                header::LOCATION,
                format!("/login?{query_string}&tag={hmac_tag:x}"),
            ))
            .finish()
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::SEE_OTHER
    }
}

#[tracing::instrument(
    skip(form, pool, session),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
    )]
pub async fn login(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    session: TypedSession,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = Credentials {
        username: form.0.username,
        password: form.0.password,
    };
    match validate_credentials(credentials, &pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", tracing::field::display(&user_id));

            session.renew();
            session
                .insert_user_id(user_id)
                .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))?;

            Ok(HttpResponse::SeeOther()
                .insert_header((header::LOCATION, "/admin/dashboard"))
                .finish())
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };

            Err(login_redirect(e))
        }
    }
}

fn login_redirect(e: LoginError) -> InternalError<LoginError> {
    FlashMessage::error(e.to_string()).send();
    let response = HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/login"))
        .finish();
    InternalError::from_response(e, response)
}
