use std::collections::HashMap;

pub struct TestEnvironmentInstance {
    pub tables: Vec<VMTable>,
    /* Each table has it's own record pointer, as per FileMaker */
    pub record_ptrs: Vec<Option<Record>>,
    pub instruction_ptr: usize,
}

impl TestEnvironmentInstance {
    pub fn new() -> Self {
        Self {
            tables: vec![],
            record_ptrs: vec![],
            instruction_ptr: 0,
        }
    }
}

pub struct VMTable {
    pub name: String,
    pub records: HashMap<String, Vec<String>>,
}

type Record = usize;
