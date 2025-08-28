use serde::{Deserialize, Serialize};

use crate::reader::dtos::{aisle_dto::AisleDTO, edge_dto::EdgeDTO, robot_dto::RobotDTO};

#[derive(Debug, Deserialize, Serialize)]
pub struct WarehouseDTO {
    pub aisles: Vec<AisleDTO>,
    pub edges: Vec<EdgeDTO>,
    pub robots: Vec<RobotDTO>,
}
