#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let settings = nexis_rs::settings::get_settings().expect("Failed to read settings.");

    let base_url = settings.application.base_url.clone();

    let subscriber = nexis_rs::telemetry::get_subscriber(settings.clone().debug);
    nexis_rs::telemetry::init_subscriber(subscriber);

    let _ = &*nexis_rs::ENV;

    let application = nexis_rs::startup::Application::build(settings, None).await?;

    tracing::info!(target: "backend", "Listening on {}:{}/", base_url, application.port());

    application.run_until_stopped().await?;

    Ok(())
}