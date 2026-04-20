use event_relay::{Config, Result, init_tracing};
use tracing::{debug, info};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;

    init_tracing(&config.rust_log);

    info!("Starting Event Relay service...");
    debug!(config = ?config, "Loaded configuration");

    // TODO(pencelheimer): connect to the DB and RMQ

    info!("Service initialized successfully. Waiting for events...");

    // NOTE(pencelheimer): unwrap is ok, because we are shutting down
    tokio::signal::ctrl_c().await.unwrap();
    info!("Shutting down...");

    Ok(())
}
