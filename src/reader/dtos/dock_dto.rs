use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DockDTO {
    pub storage_capacity: u16,
    pub capacity: u8,
    pub occupants: Vec<String>,
}
