use std::net::TcpListener;

use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::{
    configuration::{DatabaseSettings, get_configuration},
    get_subscriber, init_subscriber,
};

const TEST_DATABASE_NAME: &str = "emails_test";

static TRACING: Lazy<()> = Lazy::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber("test".into(), "debug".into(), std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber("test".into(), "debug".into(), std::io::sink);
        init_subscriber(subscriber);
    }
});

#[allow(dead_code)]
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

impl TestApp {
    pub async fn new() -> Self {
        Lazy::force(&TRACING);

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
    PgPool::connect(config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.")
}
