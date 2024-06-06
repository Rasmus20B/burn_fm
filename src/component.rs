
#[derive(Debug, Clone)]
pub enum FMComponentType {
    Table,
    Field,
    Layout
}

#[derive(Debug, Clone)]
pub struct FMComponentField {
    ctype: FMComponentType,
    field_name: String,
    field_type: String,
    created_by: String,
    modified_by: String,
}

#[derive(Debug, Clone)]
pub struct FMComponentTable {
    ctype: FMComponentType,
    table_name : String,
    fields: Vec<FMComponentField>
}

pub enum VecWrapper {
    Tables(Vec<FMComponentTable>),
    Fields(Vec<FMComponentField>)
}
