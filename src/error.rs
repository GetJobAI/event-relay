use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error setting the environment: {0}")]
    Config(#[from] envy::Error),

    #[error("Error interacting with the DB: {0}")]
    DB(#[from] sqlx::Error),

    #[error("Error interacting with the Notifications Broker: {0}")]
    Broker(#[from] lapin::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
