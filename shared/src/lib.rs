use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Session {
    pub id: u64,
    pub username: String,
}
