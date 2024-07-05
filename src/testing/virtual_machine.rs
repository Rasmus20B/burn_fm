use std::collections::BTreeMap;

pub struct VirtualMachine {
    pub tables: Vec<VMTable>,
    /* Each table has it's own record pointer, as per FileMaker */
    pub record_ptrs: Vec<Record>
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            tables: vec![],
            record_ptrs: vec![],
        }
    }
}

pub struct VMTable {
    pub records: BTreeMap<String, Vec<String>>,
}

type Record = usize;
