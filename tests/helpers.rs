use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::configuration::{DatabaseSettings, get_configuration};

const TEST_DATABASE_NAME: &str = "emails_test";

#[allow(dead_code)]
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

impl TestApp {
    pub async fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let mut configuration = get_configuration().expect("Failed to read configuration.");
        configuration.database.database_name = TEST_DATABASE_NAME.to_owned();
        let connection_pool = configure_database(&configuration.database).await;
        let server = zero2prod::startup::run(listener, connection_pool.clone())
            .expect("Failed to bind address");
        tokio::spawn(server);
        // Create a cleanup handle using tokio::spawn with a new runtime

        TestApp {
            address: format!("http://127.0.0.1:{port}"),
            db_pool: connection_pool,
        }
    }
}

pub async fn spawn_app() -> TestApp {
    TestApp::new().await
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.")
}
