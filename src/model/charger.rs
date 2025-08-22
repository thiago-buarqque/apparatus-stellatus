use std::rc::Rc;

use crate::model::location::Location;

pub struct Charger {
    pub id: u8,
    pub location: Rc<Location>
}
