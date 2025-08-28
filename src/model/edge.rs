use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::model::{aisle::Aisle, location::Location, robot::Robot};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum EdgeType {
    Normal,
    ChargeStation,
    Dock,
    Shelf,
}

pub struct Edge {
    aisles: Vec<Rc<RefCell<Aisle>>>,
    capacity: u8,
    coords: [[u8; 2]; 2],
    edge_type: EdgeType,
    id: String,
    occupants: Vec<Rc<RefCell<Robot>>>,
}

impl Location for Edge {
    fn get_coords(&self) -> &[[u8; 2]; 2] {
        &self.coords
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}

impl Edge {
    pub fn new(
        aisles: Vec<Rc<RefCell<Aisle>>>,
        capacity: u8,
        coords: [[u8; 2]; 2],
        edge_type: EdgeType,
        id: String,
        occupants: Vec<Rc<RefCell<Robot>>>,
    ) -> Self {
        Self {
            aisles,
            capacity,
            coords,
            edge_type,
            id,
            occupants,
        }
    }

    pub fn add_aisle(&mut self, aisle: Rc<RefCell<Aisle>>) {
        self.aisles.push(aisle)
    }

    pub fn add_occupant(&mut self, occupant: Rc<RefCell<Robot>>) {
        self.occupants.push(occupant)
    }
}
