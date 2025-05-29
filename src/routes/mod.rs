mod health_check;
mod newsletter;
mod subscriptions;
mod subscriptions_confirm;

pub use health_check::health_check;
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
