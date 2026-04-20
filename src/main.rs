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
        let event = db_listener.next_event().await?;
        debug!(payload = ?event, "Received event");

        let routing_key = event.to_string();
        if let Err(e) = rmq_publisher.publish_event(&routing_key, &event).await {
            error!(error = %e, routing_key, "Failed to publish event to RabbitMQ");
            continue;
        }

        info!("Relayed event successfully");
    }
}
