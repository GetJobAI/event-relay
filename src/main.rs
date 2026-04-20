use event_relay::{Broker, Config, DbListener, Result, init_tracing};
use tracing::{debug, error, info};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;

    init_tracing(&config.rust_log);

    info!("Starting Event Relay service...");
    debug!(config = ?config, "Loaded configuration");

    let mut db_listener = DbListener::connect(&config.db_url, "db_events").await?;
    let rmq_publisher = Broker::connect(&config.rmq_url, "db_events_exchange").await?;

    info!("Service initialized successfully. Waiting for events...");
    loop {
        let payload = db_listener.next_event().await?;
        debug!(payload = payload, "Received event",);

        // TODO(pencelheimer): parse payload to set routing_key dynamically
        let routing_key = "database.event";
        if let Err(e) = rmq_publisher.publish_event(routing_key, &payload).await {
            error!(error = %e, "Failed to publish event to RabbitMQ");
            continue;
        }

        info!("Relayed event successfully");
    }
}
