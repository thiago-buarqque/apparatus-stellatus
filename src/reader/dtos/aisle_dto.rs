use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AisleDTO {
    pub id: String,
    pub capacity: u8,
    pub occupants: Vec<String>,
    pub coords: [[u8; 2]; 2],
    pub from: String,
    pub to: String,
    pub parent_aisle: Option<String>,
    pub distance: f32,
}
