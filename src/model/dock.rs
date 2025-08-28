use std::rc::Rc;

use crate::model::robot::Robot;

pub struct Dock {
    pub storage_capacity: u16,
    pub capacity: u8,
    pub occupants: Vec<Rc<Robot>>,
}
