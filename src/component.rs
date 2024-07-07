
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::fm_script_engine::fm_script_engine_instructions::ScriptStep;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FMComponentType {
    Table,
    Field,
    Layout,
    Script,
    TableOccurence,
    Relationship
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentField {
    pub data_type: String,
    pub field_description: String,
    pub field_name: String,
    pub field_type: String,
    pub created_by_account: String,
    pub created_by_user: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentTable {
    pub table_name: String,
    pub created_by_account: String,
    pub create_by_user: String,
    pub fields: HashMap<u16, FMComponentField>
}

impl FMComponentTable {
    pub fn new() -> Self {
        Self {
            table_name: String::new(),
            created_by_account: String::new(),
            create_by_user: String::new(),
            fields: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentScript {
    pub script_name: String,
    pub created_by_account: String,
    pub create_by_user: String,
    pub arguments: Vec<String>,
    pub instructions: Vec<ScriptStep>,
} 

impl FMComponentScript {
    pub fn new() -> Self {
        Self {
            script_name: String::new(),
            created_by_account: String::new(),
            create_by_user: String::new(),
            arguments: vec![],
            instructions: vec![]
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentTableOccurence {
    pub table_occurence_name: String,
    pub table_actual: u16,
    pub created_by_account: String,
    pub create_by_user: String,
}

impl FMComponentTableOccurence {
    pub fn new() -> Self {
        Self {
            table_occurence_name: String::new(),
            table_actual: 0,
            created_by_account: String::new(),
            create_by_user: String::new()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentRelationship {
    pub table1: u16,
    pub table2: u16,
    pub comparison: u8,
}

impl FMComponentRelationship {
    pub fn new() -> Self {
        Self {
            table1: 0,
            table2: 0,
            comparison: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentLayout {
    pub layout_name: String,
    pub created_by_account: String,
    pub create_by_user: String,
}

impl FMComponentLayout {
    pub fn new() -> Self {
        Self {
            layout_name: String::new(),
            created_by_account: String::new(),
            create_by_user: String::new()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentValueList {
    pub list_name: String,
    pub created_by_account: String,
    pub create_by_user: String,
}

impl FMComponentValueList {
    pub fn new() -> Self {
        Self {
            list_name: String::new(),
            created_by_account: String::new(),
            create_by_user: String::new(),
        }
    }
}
