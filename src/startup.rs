use std::net::TcpListener;

use actix_web::{App, HttpServer, dev::Server, middleware::Logger, web};
use sqlx::PgPool;

use crate::routes::{health_check, subscriptions};

pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
