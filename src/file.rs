use crate::component;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct FmpFile {
    pub name: String,
    pub tables: Vec<component::FMComponentTable>
}
