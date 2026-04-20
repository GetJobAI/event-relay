use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct DbEvent {
    pub table: String,
    pub action: String,
    pub id: String,
    pub timestamp: String,
}

impl fmt::Display for DbEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.table, self.action.to_lowercase())
    }
}
