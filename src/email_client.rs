use reqwest::{Client, Url};
use secrecy::{ExposeSecret, SecretString};

use crate::domain::SubscriberEmail;

#[derive(Clone)]
pub struct EmailClient {
    http_client: Client,
    base_url: Url,
    sender: SubscriberEmail,
    authorization_token: SecretString,
}

#[derive(serde::Serialize)]
struct SendEmailRequest {
    from: String,
    to: String,
    subject: String,
    html_body: String,
    text_body: String,
}

impl EmailClient {
    pub fn new(base_url: Url, sender: SubscriberEmail, authorization_token: SecretString) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
            authorization_token,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), reqwest::Error> {
        let url = self
            .base_url
            .join("email")
            .expect("Failed to join base URL with email path");
        let request_body = SendEmailRequest {
            from: self.sender.as_ref().to_owned(),
            to: recipient.as_ref().to_owned(),
            subject: subject.to_owned(),
            html_body: html_content.to_owned(),
            text_body: text_content.to_owned(),
        };
        self.http_client
            .post(url)
            .header(
                "X-Postmark-Server-Token",
                self.authorization_token.expose_secret(),
            )
            .json(&request_body)
            .send()
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use fake::{
        Fake, Faker,
        faker::{
            internet::en::SafeEmail,
            lorem::en::{Paragraph, Sentence},
        },
    };
    use secrecy::SecretString;
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::any};

    use super::EmailClient;
    use crate::domain::SubscriberEmail;

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        // Arrange
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake::<String>().as_str()).unwrap();
        let email_client = EmailClient::new(
            mock_server.uri().parse().unwrap(),
            sender,
            SecretString::new(Faker.fake::<String>().into()),
        );

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email =
            SubscriberEmail::parse(SafeEmail().fake::<String>().as_str()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        // Act
        let _ = email_client
            .send_email(subscriber_email, &subject, &content, &content)
            .await;

        // Assert
    }
}
