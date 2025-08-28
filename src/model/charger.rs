use std::rc::Rc;

use crate::model::{location::Location, robot::Robot};

pub struct Charger {
    pub id: String,
    pub location: Rc<dyn Location>,
    pub occupant: Rc<Robot>,
    pub busy_since: u64, // time in milliseconds
}
