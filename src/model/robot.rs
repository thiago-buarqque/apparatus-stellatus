use std::rc::Rc;

use crate::model::{battery::Battery, location::Location};

pub struct Robot {
    pub id: u8,
    pub battery: Box<Battery>,
    pub start_location: Rc<dyn Location>,
    pub current_location: Rc<dyn Location>,
    pub speed: f32,
}
