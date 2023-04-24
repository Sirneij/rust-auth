#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    dotenv::from_filename(".env.development").ok();

    let settings = backend::settings::get_settings().expect("Failed to read settings.");

    let subscriber = backend::telemetry::get_subscriber(settings.clone().debug);
    backend::telemetry::init_subscriber(subscriber);

    let application = backend::startup::Application::build(settings, None).await?;

    tracing::event!(target: "backend", tracing::Level::INFO, "Listening on http://127.0.0.1:{}/", application.port());

    application.run_until_stopped().await?;
    Ok(())
}
