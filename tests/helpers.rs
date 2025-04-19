use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::configuration::get_configuration;

#[allow(dead_code)]
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let server =
        zero2prod::startup::run(listener, connection_pool.clone()).expect("Failed to bind address");
    tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{port}"),
        db_pool: connection_pool,
    }
}
