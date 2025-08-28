use crate::reader::parser::Parser;

mod model;
mod reader;

fn main() {
    let mut parser = Parser::default();

    let _ = parser.read_json("./src/resources/sample.json");
}
