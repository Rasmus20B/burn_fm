use std::collections::BTreeMap;

pub struct VirtualMachine {
    tables: Vec<VMTable>,
    /* Each table has it's own record pointer, as per FileMaker */
    record_ptrs: Vec<record>
}

pub struct VMTable {
    records: BTreeMap<String, Vec<String>>,
}

type record = usize;
