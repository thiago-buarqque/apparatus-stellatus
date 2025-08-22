use std::rc::Rc;

use crate::model::{corner::Corner, location::Location, robot::Robot};

pub struct Aisle {
    id: u16,
    capacity: u8,
    occupants: Vec<Rc<Robot>>,
    coords: [[u8; 2]; 2],
    from: Rc<Corner>,
    to: Rc<Corner>,
    distance: f32
}

impl Location for Aisle {
    fn get_coords(&self) -> &[[u8; 2]; 2] {
        &self.coords
    }

    fn get_id(&self) -> &u16 {
        &self.id
    }
}
