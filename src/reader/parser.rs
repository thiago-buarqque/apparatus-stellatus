use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::model::{aisle::Aisle, edge::Edge, robot::Robot};

#[derive(Default)]
pub struct Parser {
    edges: HashMap<String, Edge>,
    aisles: HashMap<String, Aisle>,
    robots: HashMap<String, Robot>,
}

impl Parser {
    pub fn read_json(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_str = match std::fs::read_to_string(path) {
            Ok(json) => json,
            Err(error) => panic!("{}", error)
        };

        let parsed: Value = serde_json::from_str(&json_str)?;
        let obj: Map<String, Value> = parsed.as_object().unwrap().clone();

        println!("{:#?}", obj);

        Ok(())
    }
}
