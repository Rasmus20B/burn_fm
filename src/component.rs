
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::script_engine::script_engine_instructions::Instruction;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentScript {
    pub script_name: String,
    pub created_by_account: String,
    pub create_by_user: String,
    pub instructions: Vec<Instruction>,
} 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentTableOccurence {
    pub table_occurence_name: String,
    pub table_actual: u16,
    pub created_by_account: String,
    pub create_by_user: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentRelationship {
    pub table1: u16,
    pub table2: u16,
    pub comparison: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentLayout {
    pub layout_name: String,
    pub created_by_account: String,
    pub create_by_user: String,
}

