use std::net::TcpListener;

use sqlx::{Connection, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::configuration::{DatabaseSettings, get_configuration};

#[allow(dead_code)]
pub struct TestApp {
    pub db_config: DatabaseSettings,
    pub address: String,
    pub db_pool: PgPool,
    pub cleanup_handle: Option<std::thread::JoinHandle<()>>,
}

impl TestApp {
    pub async fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let mut configuration = get_configuration().expect("Failed to read configuration.");
        configuration.database.database_name = format!("emails_{}", Uuid::new_v4());
        let connection_pool = configure_database(&configuration.database).await;
        let server = zero2prod::startup::run(listener, connection_pool.clone())
            .expect("Failed to bind address");
        tokio::spawn(server);
        // Create a cleanup handle using tokio::spawn with a new runtime
        let db_config = configuration.database.clone();
        let db_pool = connection_pool.clone();
        let cleanup_handle = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to build runtime for cleanup");
            
            rt.block_on(async move {
                db_pool.close().await;
                // Connect to Postgres without the specific database
                let mut connection = PgConnection::connect(&db_config.connection_string_without_db())
                    .await
                    .expect("Failed to connect to Postgres");
                
                // Drop the database
                sqlx::query(&format!(r#"DROP DATABASE IF EXISTS "{}";"#, db_config.database_name))
                    .execute(&mut connection)
                    .await
                    .expect("Failed to drop database");
                
                println!("Dropped database: {}", db_config.database_name);
            });
        });

        TestApp {
            address: format!("http://127.0.0.1:{port}"),
            db_config: configuration.database,
            db_pool: connection_pool,
            cleanup_handle: Some(cleanup_handle),
        }
    }

    // pub async fn cleanup(&self) {
    //     sqlx::query(&format!(
    //         "DROP DATABASE IF EXISTS \"{}\";",
    //         self.db_config.database_name
    //     ))
    //     .execute(&self.db_pool)
    //     .await
    //     .expect("Failed to drop database.");
    //     println!("Dropped database: {}", self.db_config.database_name);
    // }
}

pub async fn spawn_app() -> TestApp {
    TestApp::new().await
}

impl Drop for TestApp {
    fn drop(&mut self) {
        if let Some(handle) = self.cleanup_handle.take() {
            handle.join().expect("cleanup thread panicked");
        }
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::query(&format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .execute(&mut connection)
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    println!("Migrated database: {}", config.database_name);

    connection_pool
}
