use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BatteryDTO {
    pub capacity: f32,
    pub per_meter: f32,
    pub per_pick: f32,
    pub per_turn: f32,
    pub per_minute: f32,
}
