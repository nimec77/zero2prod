use tokio::task::JoinError;
use zero2prod::issue_delivery_worker::run_worker_until_stopped;
use zero2prod::{
    configuration::get_configuration, get_subscriber, init_subscriber, startup::Application,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration.clone()).await?;
    let application_task = tokio::spawn(application.run_until_stopped());
    let worker_task = tokio::spawn(run_worker_until_stopped(configuration));

    tokio::select! {
        o = application_task => report_exit("API", o),
        o = worker_task => report_exit("Background worker", o),
    };

    Ok(())
}

fn report_exit<E>(task_name: &str, outcome: Result<Result<(), E>, JoinError>)
where
    E: std::fmt::Debug + std::fmt::Display,
{
    match outcome {
        Ok(Ok(())) => {
            tracing::info!(task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
            error.cause_chain = ?e,
            error.message = %e,
            task_name
            )
        }
        Err(e) => {
            tracing::error!(
            error.cause_chain = ?e,
            error.message = %e,
            task_name
            )
        }
    }
}
