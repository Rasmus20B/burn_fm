
use std::collections::HashMap;

use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FMComponentType {
    Table,
    Field,
    Layout
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
pub struct FMComponentRelationship {
    pub table1: u16,
    pub table2: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMComponentLayout {
    pub layout_name: String,
}

pub enum VecWrapper {
    Tables(Vec<FMComponentTable>),
    Fields(Vec<FMComponentField>)
}
