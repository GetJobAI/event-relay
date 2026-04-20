use crate::error::Result;
use lapin::{
    BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind,
    options::{BasicPublishOptions, ExchangeDeclareOptions},
    types::FieldTable,
};
use tracing::instrument;

pub struct Broker {
    channel: Channel,
    exchange: String,

    _connection: Connection,
}

impl Broker {
    #[instrument(
        skip_all,
        fields(exchange = exchange.as_ref())
    )]
    pub async fn connect(amqp_url: impl AsRef<str>, exchange: impl AsRef<str>) -> Result<Self> {
        let amqp_url = amqp_url.as_ref();
        let exchange = exchange.as_ref().to_string();

        let connection = Connection::connect(amqp_url, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;

        channel
            .exchange_declare(
                exchange.clone().into(),
                ExchangeKind::Topic,
                ExchangeDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        Ok(Self {
            channel,
            exchange,

            _connection: connection,
        })
    }

    #[instrument(
        skip_all,
        fields(routing_key = %routing_key.as_ref())
    )]
    pub async fn publish_event(
        &self,
        routing_key: impl AsRef<str>,
        payload: impl AsRef<str>,
    ) -> Result<()> {
        let routing_key = routing_key.as_ref();
        let payload = payload.as_ref();

        self.channel
            .basic_publish(
                self.exchange.clone().into(),
                routing_key.into(),
                BasicPublishOptions::default(),
                payload.as_bytes(),
                BasicProperties::default().with_delivery_mode(2), // NOTE(pencelheimer): 2 = Persistent
            )
            .await?;

        Ok(())
    }
}
