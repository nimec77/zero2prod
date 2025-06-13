mod admin;
mod health_check;
mod home;
mod login;
mod newsletter;
mod subscriptions;
mod subscriptions_confirm;

pub use admin::*;
pub use health_check::health_check;
pub use home::home;
pub use login::{login, login_form};
pub use newsletter::publish_newsletter;
pub use subscriptions::subscribe;
pub use subscriptions_confirm::confirm;

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{e}\n")?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{cause}")?;
        current = cause.source();
    }
    Ok(())
}
