
#[derive(Debug, Clone)]
pub enum FMComponentType {
    Table,
    Field,
    Layout
}

#[derive(Debug, Clone)]
pub struct FMComponentField<'a> {
    ctype: FMComponentType,
    field_name: String,
    field_type: String,
    created_by: String,
    modified_by: String,
    table: &'a FMComponentTable
}

#[derive(Debug, Clone)]
pub struct FMComponentTable {
    ctype: FMComponentType,
    table_name : String,
    n_fields : usize
}

pub enum VecWrapper<'a> {
    Tables(Vec<FMComponentTable>),
    Fields(Vec<FMComponentField<'a>>)
}
