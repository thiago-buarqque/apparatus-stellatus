use serde::{Deserialize, Serialize};

use crate::{model::battery::Battery, reader::dtos::battery_dto::BatteryDTO};

#[derive(Debug, Deserialize, Serialize)]
pub struct RobotDTO {
    pub id: String,
    pub battery: Battery,
    pub start_location: String,
    pub current_location: String,
    pub speed: f32,
}
