use serde::{Deserialize, Serialize};

use crate::model::edge::EdgeType;

#[derive(Debug, Deserialize, Serialize)]
pub struct EdgeDTO {
    pub aisles: Vec<String>,
    pub capacity: u8,
    pub coords: [[u8; 2]; 2],
    pub edge_type: EdgeType,
    pub id: String,
    pub occupants: Vec<String>,
}
