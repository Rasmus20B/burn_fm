
use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::fm_script_engine::fm_script_engine_instructions::ScriptStep;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FMComponentType {
    Table,
    Field,
    Layout,
    Script,
    TableOccurence,
    Relationship,
    Test,
    DataSource,
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
impl FMComponentField {
    pub fn new() -> Self {
        Self {
            data_type: String::new(),
            field_description: String::new(),
            field_name: String::new(),
            field_type: String::new(),
            created_by_account: String::new(),
            created_by_user: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentTest {
    pub test_name: String,
    pub script: FMComponentScript,
    pub created_by_account: String,
    pub create_by_user: String,
    pub assertions: Vec<String>,
}

impl FMComponentTest {
    pub fn new() -> Self {
        Self {
            test_name: String::new(),
            script: FMComponentScript::new(),
            created_by_account: String::new(),
            create_by_user: String::new(),
            assertions: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentTable {
    pub table_name: String,
    pub created_by_account: String,
    pub create_by_user: String,
    pub fields: BTreeMap<u16, FMComponentField>,
    pub init: bool,
}

impl FMComponentTable {
    pub fn new() -> Self {
        Self {
            table_name: String::new(),
            created_by_account: String::new(),
            create_by_user: String::new(),
            fields: BTreeMap::new(),
            init: false
        }
    }

    pub fn new_init() -> Self {
        Self {
            table_name: String::new(),
            created_by_account: String::new(),
            create_by_user: String::new(),
            fields: BTreeMap::new(),
            init: true
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
            instructions: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentTableOccurence {
    pub table_occurence_name: String,
    pub table_actual: u16,
    pub table_actual_name: String,
    pub created_by_account: String,
    pub create_by_user: String,
}

impl FMComponentTableOccurence {
    pub fn new() -> Self {
        Self {
            table_occurence_name: String::new(),
            table_actual: 0,
            table_actual_name: String::new(),
            created_by_account: String::new(),
            create_by_user: String::new()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationComparison {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Cartesian
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentRelationship {
    pub table1: u16,
    pub table1_name: String,
    pub field1: u16,
    pub table2: u16,
    pub table2_name: String,
    pub field2: u16,
    pub comparison: RelationComparison,
}

impl FMComponentRelationship {
    pub fn new() -> Self {
        Self {
            table1: 0,
            table1_name: String::new(),
            field1: 0,
            table2: 0,
            table2_name: String::new(),
            field2: 0,
            comparison: RelationComparison::Equal,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentLayout {
    pub layout_name: String,
    pub table_occurrence: usize,
    pub created_by_account: String,
    pub create_by_user: String,
}

impl FMComponentLayout {
    pub fn new() -> Self {
        Self {
            layout_name: String::new(),
            table_occurrence: 0,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentDataSource {
    pub source_name: String,
    pub source_type: String,
    pub source_path: String,
    pub source_filename: String,
    pub created_by_account: String,
    pub created_by_user: String,
}

impl FMComponentDataSource {
    pub fn new() -> Self {
        Self {
            source_name: String::new(),
            source_type: String::new(),
            source_path: String::new(),
            source_filename: String::new(),
            created_by_account: String::new(),
            created_by_user: String::new(),
        }
    }
}
