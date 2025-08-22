use std::rc::Rc;

use crate::model::{aisle::Aisle, location::Location, robot::Robot};

pub struct Corner {
    id: u16,
    ailes: Vec<Rc<Aisle>>,
    capacity: u8,
    coords: [[u8; 2]; 2],
    occupants: Vec<Rc<Robot>>,
}

impl Location for Corner {
    fn get_coords(&self) -> &[[u8; 2]; 2] {
        &self.coords
    }

    fn get_id(&self) -> &u16 {
        &self.id
    }
}
