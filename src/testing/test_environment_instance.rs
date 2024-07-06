use std::collections::BTreeMap;

pub struct TestEnvironmentInstance {
    pub tables: Vec<VMTable>,
    /* Each table has it's own record pointer, as per FileMaker */
    pub record_ptrs: Vec<Option<Record>>
}

impl TestEnvironmentInstance {
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
