use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, web};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(("127.0.0.1", 8000))?
        .run();

    Ok(server)
}

#[cfg(test)]
mod tests {

    use actix_web::{Responder, test};

    use crate::health_check;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check()
            .await
            .respond_to(&test::TestRequest::default().to_http_request());

        assert!(response.status().is_success());
    }
}
