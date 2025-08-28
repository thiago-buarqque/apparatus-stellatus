use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    model::{aisle::Aisle, edge::Edge, robot::Robot},
    reader::dtos::{
        aisle_dto::AisleDTO, edge_dto::EdgeDTO, robot_dto::RobotDTO,
        warehouse_dto::WarehouseDTO,
    },
};

#[derive(Default)]
pub struct Warehouse {
    edges: HashMap<String, Rc<RefCell<Edge>>>,
    aisles: HashMap<String, Rc<RefCell<Aisle>>>,
    robots: HashMap<String, Rc<RefCell<Robot>>>,
}

impl Warehouse {
    pub fn from_json_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json_str = std::fs::read_to_string(path)?;
        let warehouse_dto: WarehouseDTO = serde_json::from_str(&json_str)?;

        Self::from_dto(warehouse_dto)
    }

    fn from_dto(warehouse_dto: WarehouseDTO) -> Result<Self, Box<dyn std::error::Error>> {
        let mut warehouse = Warehouse::default();

        warehouse.create_edges_from_dto(&warehouse_dto.edges);
        warehouse.create_robots_from_dto(&warehouse_dto.robots)?;
        warehouse.create_aisles_from_dto(&warehouse_dto.aisles)?;

        warehouse.link_edges_to_components(&warehouse_dto.edges)?;

        Ok(warehouse)
    }

    fn create_edges_from_dto(&mut self, edge_dtos: &[EdgeDTO]) {
        for edge_dto in edge_dtos {
            let edge = Edge::new(
                Vec::new(), // Aisles linked in link_edges_to_components
                edge_dto.capacity,
                edge_dto.coords,
                edge_dto.edge_type,
                edge_dto.id.clone(),
                Vec::new(), // Occupants linked in link_edges_to_components
            );
            self.edges
                .insert(edge_dto.id.clone(), Rc::new(RefCell::new(edge)));
        }
    }

    fn create_robots_from_dto(
        &mut self,
        robot_dtos: &[RobotDTO],
    ) -> Result<(), String> {
        for robot_dto in robot_dtos {
            let current_location = self
                .edges
                .get(&robot_dto.current_location)
                .ok_or_else(|| {
                    format!(
                        "Invalid edge ID '{}' for robot '{}' current_location.",
                        robot_dto.current_location, robot_dto.id
                    )
                })?
                .clone();

            let start_location = self
                .edges
                .get(&robot_dto.start_location)
                .ok_or_else(|| {
                    format!(
                        "Invalid edge ID '{}' for robot '{}' start_location.",
                        robot_dto.start_location, robot_dto.id
                    )
                })?
                .clone();

            let robot = Robot::new(
                Box::new(RefCell::new(robot_dto.battery)),
                current_location,
                robot_dto.id.clone(),
                robot_dto.speed,
                start_location,
            );
            self.robots
                .insert(robot_dto.id.clone(), Rc::new(RefCell::new(robot)));
        }
        Ok(())
    }

    fn create_aisles_from_dto(
        &mut self,
        aisle_dtos: &[AisleDTO],
    ) -> Result<(), String> {
        for aisle_dto in aisle_dtos {
            let from_edge = self.edges.get(&aisle_dto.from).ok_or_else(|| {
                format!(
                    "Invalid 'from' edge ID '{}' for aisle '{}'.",
                    aisle_dto.from, aisle_dto.id
                )
            })?;
            let to_edge = self.edges.get(&aisle_dto.to).ok_or_else(|| {
                format!(
                    "Invalid 'to' edge ID '{}' for aisle '{}'.",
                    aisle_dto.to, aisle_dto.id
                )
            })?;

            let occupants = aisle_dto
                .occupants
                .iter()
                .map(|occupant_id| {
                    self.robots.get(occupant_id).cloned().ok_or_else(|| {
                        format!(
                            "Invalid occupant ID '{}' for aisle '{}'.",
                            occupant_id, aisle_dto.id
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;

            let aisle = Aisle::new(
                aisle_dto.capacity,
                aisle_dto.coords,
                aisle_dto.distance,
                from_edge.clone(),
                aisle_dto.id.clone(),
                occupants,
                None,
                to_edge.clone(),
            );
            self.aisles
                .insert(aisle_dto.id.clone(), Rc::new(RefCell::new(aisle)));
        }
        Ok(())
    }

    fn link_edges_to_components(
        &mut self,
        edge_dtos: &[EdgeDTO],
    ) -> Result<(), String> {
        for edge_dto in edge_dtos {
            let edge = self.edges.get(&edge_dto.id).ok_or_else(|| {
                format!("Consistency error: Edge with ID '{}' not found during linking pass.", edge_dto.id)
            })?;

            for aisle_id in &edge_dto.aisles {
                let aisle = self.aisles.get(aisle_id).ok_or_else(|| {
                    format!("Invalid aisle ID '{}' in edge '{}'.", aisle_id, edge_dto.id)
                })?;
                edge.borrow_mut().add_aisle(Rc::clone(aisle));
            }

            for occupant_id in &edge_dto.occupants {
                let robot = self.robots.get(occupant_id).ok_or_else(|| {
                    format!("Invalid occupant ID '{}' in edge '{}'.", occupant_id, edge_dto.id)
                })?;
                edge.borrow_mut().add_occupant(Rc::clone(robot));
            }
        }
        Ok(())
    }
}
