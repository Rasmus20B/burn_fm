use crate::component;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct FmpFile {
    pub name: String,
    pub tables: HashMap<usize, component::FMComponentTable>,
    pub relationships: HashMap<usize, component::FMComponentRelationship>,
    pub layouts: HashMap<usize, component::FMComponentLayout>,
    pub scripts: HashMap<usize, component::FMComponentScript>,
}
