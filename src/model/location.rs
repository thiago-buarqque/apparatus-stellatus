pub trait Location {
    fn get_coords(&self) -> &[[u8; 2]; 2];
    fn get_id(&self) -> &str;
}
