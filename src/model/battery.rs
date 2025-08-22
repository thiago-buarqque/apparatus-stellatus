#[derive(Debug, PartialEq)]
pub struct Battery {
    pub capacity: f32,
    pub per_meter: f32,
    pub per_pick: f32,
    pub per_turn: f32,
    pub per_minute: f32
}
