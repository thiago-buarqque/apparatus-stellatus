use std::rc::Rc;

use crate::model::{edge::Edge, location::Location, robot::Robot};

pub struct Aisle {
    id: String,
    capacity: u8,
    occupants: Vec<Rc<Robot>>,
    coords: [[u8; 2]; 2],
    from: Rc<Edge>,
    to: Rc<Edge>,
    parent_aisle: Option<Rc<Aisle>>,
    distance: f32,
}

impl Location for Aisle {
    fn get_coords(&self) -> &[[u8; 2]; 2] {
        &self.coords
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}
