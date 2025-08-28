use std::{cell::RefCell, rc::Rc};

use crate::model::{edge::Edge, location::Location, robot::Robot};

pub struct Aisle {
    capacity: u8,
    coords: [[u8; 2]; 2],
    distance: f32,
    from: Rc<RefCell<Edge>>,
    id: String,
    occupants: Vec<Rc<RefCell<Robot>>>,
    parent_aisle: Option<Rc<RefCell<Aisle>>>,
    to: Rc<RefCell<Edge>>,
}

impl Aisle {
    pub fn new(
        capacity: u8,
        coords: [[u8; 2]; 2],
        distance: f32,
        from: Rc<RefCell<Edge>>,
        id: String,
        occupants: Vec<Rc<RefCell<Robot>>>,
        parent_aisle: Option<Rc<RefCell<Aisle>>>,
        to: Rc<RefCell<Edge>>,
    ) -> Self {
        Aisle {
            capacity,
            coords,
            distance,
            from,
            id,
            occupants,
            parent_aisle,
            to,
        }
    }
}

impl Location for Aisle {
    fn get_coords(&self) -> &[[u8; 2]; 2] {
        &self.coords
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}
