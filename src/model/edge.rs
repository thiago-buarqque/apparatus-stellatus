use std::rc::Rc;

use crate::model::{aisle::Aisle, location::Location, robot::Robot};

pub enum EdgeType {
    Normal,
    ChargeStation,
    Dock,
    Shelf,
}

pub struct Edge {
    id: String,
    ailes: Vec<Rc<Aisle>>,
    capacity: u8,
    coords: [[u8; 2]; 2],
    edge_type: EdgeType,
    occupants: Vec<Rc<Robot>>,
}

impl Location for Edge {
    fn get_coords(&self) -> &[[u8; 2]; 2] {
        &self.coords
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}
