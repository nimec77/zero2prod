use validator::Validate;

#[derive(Debug, Validate)]
pub struct SubscriberEmail {
    #[validate(email)]
    email: String,
}

impl SubscriberEmail {
    pub fn parse(s: &str) -> Result<Self, String> {
        let subscriber_email = Self {
            email: s.to_owned(),
        };
        match subscriber_email.validate() {
            Ok(_) => Ok(subscriber_email),
            Err(_) => Err(format!("{s} is not a valid subscriber email.")),
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.email
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claims::assert_err;
    use fake::{
        Fake,
        faker::internet::en::SafeEmail,
        rand::{SeedableRng, rngs::StdRng},
    };
    use proptest::prelude::*;

    /// Strategy that generates “safe” emails by seeding Fake’s RNG from a u64.
    fn email_strategy() -> impl Strategy<Value = String> {
        any::<u64>() // draw a 64-bit seed from Proptest  [oai_citation:2‡Docs.rs](https://docs.rs/proptest/latest/proptest/prelude/index.html?utm_source=chatgpt.com)
            .prop_map(|seed| {
                let mut rng = StdRng::seed_from_u64(seed); // seed the RNG  [oai_citation:3‡Docs.rs](https://docs.rs/crate/fake/latest)
                SafeEmail().fake_with_rng(&mut rng) // generate a safe email  [oai_citation:4‡Docs.rs](https://docs.rs/crate/fake/latest)
            })
    }

    #[test]
    fn empty_string_is_rejected() {
        assert_err!(SubscriberEmail::parse(""));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        assert_err!(SubscriberEmail::parse("ursuladomain.com"));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        assert_err!(SubscriberEmail::parse("@domain.com"));
    }

    proptest! {
        /// Every generated email should pass `validate_email`.
        #[test]
        fn valid_emails_pass(email in email_strategy()) {
            dbg!(&email);
            prop_assert!(SubscriberEmail::parse(&email).is_ok(),
                "Expected `{}` to be valid", email);
        }
    }
}
