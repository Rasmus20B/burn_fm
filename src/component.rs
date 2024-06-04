
#[derive(Debug, Clone)]
pub enum FMComponentType {
    Table,
    Field,
    Layout
}

#[derive(Debug, Clone)]
pub struct FMComponentField {
    ctype: FMComponentType,
    field_name : String,
    field_type : String
}

#[derive(Debug, Clone)]
pub struct FMComponentTable {
    ctype: FMComponentType,
    table_name : String,
    n_fields : usize
}

pub enum VecWrapper {
    Tables(Vec<FMComponentTable>),
    Fields(Vec<FMComponentField>)
}
