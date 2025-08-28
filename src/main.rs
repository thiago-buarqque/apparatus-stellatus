use crate::reader::parser::Warehouse;

mod model;
mod reader;

fn main() {
    let warehouse = Warehouse::from_json_file("./src/resources/sample.json");
}
