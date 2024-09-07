
use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::{encoding_util::fm_string_decrypt, fm_script_engine::fm_script_engine_instructions::ScriptStep};

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
pub struct SourceFileLocation {
    pub source_type: String,
    pub source_path: String,
    pub source_filename: String,
}

impl SourceFileLocation {
    pub fn new() -> Self {
        Self {
            source_type: String::new(),
            source_path: String::new(),
            source_filename: String::new(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {

        let head = &bytes[0..4];
        let body = &bytes[4..];

        let path_start = body.windows(2)
            .position(|w| return w[0] == 65 && w[1] == 50).unwrap_or(0) - 1;

        let path_len = body[path_start] as usize;
        let path_bytes = &body[path_start+3..path_start+3+path_len];

        let path : Vec<_> = path_bytes.into_iter()
            .map(|b| vec![b])
            .map(|b| match b[..] {
                [50] => { vec![] }
                [65] => { vec![96 as u8]},
                [218] => { vec![(117 as u8)] },
                [219] => { vec![(116 as u8), (116 as u8)] },
                _ => b.into_iter().map(|n| *n).collect()
            })
            .flatten()
            .collect();

        let path_string = fm_string_decrypt(&path);
        let file_start = path_start+3+path_len+1;
        let file_len = body[file_start - 1] as usize;
        let filename = fm_string_decrypt(&body[file_start..file_start+file_len]);
        let mut res = Self::new();
        res.source_filename = filename;
        res.source_path = path_string;
        res.source_type = fm_string_decrypt(&body[1..path_start]);
        res

    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentDataSource {
    pub source_name: String,
    pub source_location: SourceFileLocation,
    pub created_by_account: String,
    pub created_by_user: String,
}

impl FMComponentDataSource {
    pub fn new() -> Self {
        Self {
            source_name: String::new(),
            source_location: SourceFileLocation::new(),
            created_by_account: String::new(),
            created_by_user: String::new(),
        }
    }
}
