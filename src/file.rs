use crate::component;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct FmpFile {
    pub name: String,
    pub tables: HashMap<usize, component::FMComponentTable>,
    pub relationships: HashMap<usize, component::FMComponentRelationship>,
    pub layouts: HashMap<usize, component::FMComponentLayout>,
    pub scripts: HashMap<usize, component::FMComponentScript>,
    pub table_occurrences: HashMap<usize, component::FMComponentTableOccurence>,
}

impl FmpFile {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            tables: HashMap::new(),
            relationships: HashMap::new(),
            layouts: HashMap::new(),
            scripts: HashMap::new(),
            table_occurrences: HashMap::new(),
        }
    }
}
