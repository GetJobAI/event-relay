use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing(log_level: impl AsRef<str>) {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level));

    tracing_subscriber::registry()
        .with(env_filter)
        // TODO(pencelheimer): print logs as json and collect them in one place?
        .with(fmt::layer().pretty())
        .init();
}
