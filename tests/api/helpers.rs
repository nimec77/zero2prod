use once_cell::sync::Lazy;
use sqlx::PgPool;
use zero2prod::{
    configuration::{get_configuration, DatabaseSettings},
    get_subscriber, init_subscriber, startup::{get_connection_pool, Application},
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

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let configuration = {
        let mut configuration = get_configuration().expect("Failed to read configuration.");
        configuration.database.database_name = TEST_DATABASE_NAME.to_owned();
        configuration.application.port = 0;

        configuration
    };

    configure_database(&configuration.database).await;

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();
    let address = format!("http://127.0.0.1:{application_port}");
    tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        db_pool: get_connection_pool(&configuration.database),
    }

    // let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // let port = listener.local_addr().unwrap().port();
    // let mut configuration = get_configuration().expect("Failed to read configuration.");
    // configuration.database.database_name = TEST_DATABASE_NAME.to_owned();
    // let connection_pool = configure_database(&configuration.database).await;

    // let sender_email = configuration
    //     .email_client
    //     .sender()
    //     .expect("Invalid sender email address.");
    // let timeout = configuration.email_client.timeout();
    // let email_client = EmailClient::new(
    //     configuration.email_client.base_url.parse().unwrap(),
    //     sender_email,
    //     configuration.email_client.authorization_token,
    //     timeout,
    // );

    // let server = zero2prod::startup::run(listener, connection_pool.clone(), email_client)
    //     .expect("Failed to bind address");
    // tokio::spawn(server);
    // // Create a cleanup handle using tokio::spawn with a new runtime

    // TestApp {
    //     address: format!("http://127.0.0.1:{port}"),
    //     db_pool: connection_pool,
    // }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let connection = PgPool::connect_with(config.db_options());

    connection.await.expect("Failed to connect to Postgres.")
}

pub trait UrlEncodable {
    fn url_encode(&self) -> String;
}

impl UrlEncodable for str {
    fn url_encode(&self) -> String {
        urlencoding::encode(self).into_owned()
    }
}
