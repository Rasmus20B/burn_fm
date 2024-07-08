use std::collections::HashMap;

/* TODO: check if small count and small size of VMTables means linear string comparison is faster
 * than HashMap */

pub struct TestEnvironmentInstance {
    pub tables: Vec<VMTable>,
    /* Each table has it's own record pointer, as per FileMaker */
    pub record_ptrs: Vec<Option<Record>>,
    /* Each script has it's own instruction ptr.
     * on calling of a script, a new ptr is pushed.
     * When script is finished, we pop the instruction ptr.
     * Nothing more complex than function calls. No generators etc,
     * so this is fine.
     */
    pub instruction_ptr: Vec<usize>,
}

impl TestEnvironmentInstance {
    pub fn new() -> Self {
        Self {
            tables: vec![],
            record_ptrs: vec![],
            instruction_ptr: vec![],
        }
    }
}

pub struct VMTable {
    pub name: String,
    pub records: HashMap<String, Vec<String>>,
}

type Record = usize;
