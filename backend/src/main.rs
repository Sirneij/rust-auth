#[tokio::main]
async fn main() -> std::io::Result<()> {
    let application = backend::startup::Application::build().await?;

    application.run_until_stopped().await?;
    Ok(())
}
