use std::collections::HashMap;

use crate::component::FMComponentTest;
use crate::file;
use crate::component;

use super::test_environment_instance::*;

pub struct TestEnvironment<'a> {
    pub file_handle: &'a file::FmpFile,
    pub vm: TestEnvironmentInstance,
}
impl<'a> TestEnvironment<'a> {

    pub fn new(file: &'a file::FmpFile) -> Self {
        Self {
            file_handle: file,
            vm: TestEnvironmentInstance::new(),
        }
    }

    pub fn run_tests(&mut self) {
        for test in &self.file_handle.tests {
            /* 1. Run the script 
             * 2. Check Assertions defined in test component
             * 3. Clean the test environment for next test */

        }
    }

    pub fn generate_tables_for_tests(&mut self) {
        /* For each test, we will reuse the same table structure 
         * as defined in the fmp_file. Don't rebuild for each one */
        for table in &self.file_handle.tables {
            let vmtable_tmp = VMTable {
                name: table.1.table_name.clone(),
                records: HashMap::new(),
            };
            self.vm.tables.push(vmtable_tmp);
            println!("Pushing Table \"{}\" to test environment", table.1.table_name);
            for f in &table.1.fields {
                println!("Pushing field: \"{}::{}\" to test environemnt", table.1.table_name, f.1.field_name);
                self.vm.tables.last_mut().unwrap().records
                    .insert(f.1.field_name.to_string(), vec![]);
            }
            /* Each table is empty, therefore no pointer to a record */
            self.vm.record_ptrs.push(None);
        }
    }
}


