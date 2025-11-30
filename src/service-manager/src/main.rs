use infra_runtime::ServiceManager;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    info!("Starting VRPG Service Manager");

    let _manager = ServiceManager::new();

    info!("Service Manager initialized");

    // Keep running
    tokio::signal::ctrl_c().await?;

    info!("Shutting down Service Manager");
    Ok(())
}
