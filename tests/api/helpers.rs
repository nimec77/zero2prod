use fake::{
    Fake,
    faker::{internet::en::SafeEmail, name::en::Name},
};
use once_cell::sync::Lazy;
use sqlx::PgPool;
use wiremock::MockServer;
use zero2prod::{
    configuration::{DatabaseSettings, get_configuration},
    domain::SubscriberEmail,
    get_subscriber, init_subscriber,
    startup::{Application, get_connection_pool},
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

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub email_server: MockServer,
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let email_server = MockServer::start().await;
    let configuration = {
        let mut configuration = get_configuration().expect("Failed to read configuration.");
        configuration.database.database_name = TEST_DATABASE_NAME.to_owned();
        configuration.application.port = 0;
        configuration.email_client.base_url = email_server.uri();

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
        email_server,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let connection = PgPool::connect_with(config.db_options());

    connection.await.expect("Failed to connect to Postgres.")
}

impl TestApp {
    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(format!("{}/subscribe", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub trait UrlEncodable {
    fn url_encode(&self) -> String;
}

impl UrlEncodable for str {
    fn url_encode(&self) -> String {
        urlencoding::encode(self).into_owned()
    }
}

pub fn fake_email() -> SubscriberEmail {
    SubscriberEmail::parse(SafeEmail().fake::<String>().as_str()).unwrap()
}

pub fn fake_name() -> String {
    Name().fake::<String>()
}
