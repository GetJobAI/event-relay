use crate::error::Result;
use sqlx::postgres::PgListener;
use tracing::instrument;

pub struct DbListener {
    listener: PgListener,
}

impl DbListener {
    #[instrument(skip_all, fields(channel = channel.as_ref()))]
    pub async fn connect(db_url: impl AsRef<str>, channel: impl AsRef<str>) -> Result<Self> {
        let db_url = db_url.as_ref();
        let channel_name = channel.as_ref();

        let mut listener = PgListener::connect(db_url).await?;
        listener.listen(channel_name).await?;

        Ok(Self { listener })
    }

    #[instrument(skip_all)]
    pub async fn next_event(&mut self) -> Result<String> {
        let notification = self.listener.recv().await?;

        let payload = notification.payload().to_string();
        Ok(payload)
    }
}
