use std::{cell::RefCell, rc::Rc};

use crate::model::{battery::Battery, location::Location};

pub struct Robot {
    battery: Box<RefCell<Battery>>,
    current_location: Rc<RefCell<dyn Location>>,
    id: String,
    speed: f32,
    start_location: Rc<RefCell<dyn Location>>,
}

impl Robot {
    pub fn new(
        battery: Box<RefCell<Battery>>,
        current_location: Rc<RefCell<dyn Location>>,
        id: String,
        speed: f32,
        start_location: Rc<RefCell<dyn Location>>,
    ) -> Self {
        Self {
            battery,
            current_location,
            id,
            speed,
            start_location,
        }
    }
}
