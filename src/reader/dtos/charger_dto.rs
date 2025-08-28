use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ChargerDTO {
    pub id: String,
    pub location: String,
    pub occupant: Option<String>,
    pub busy_since: u64,
}
