use std::collections::BTreeMap;

use crate::file;
use crate::component;

use super::virtual_machine;
use super::virtual_machine::VMTable;
use super::virtual_machine::VirtualMachine;

pub struct TestEnvironment<'a> {
    pub file_handle: &'a file::FmpFile,
    pub vm: virtual_machine::VirtualMachine
}
impl<'a> TestEnvironment<'a> {

    pub fn new(file: &'a file::FmpFile) -> Self {
        Self {
            file_handle: file,
            vm: VirtualMachine::new()
        }
    }

    pub fn generate_tables_for_tests(&mut self) {
        /* For each test, we will reuse the same table structure 
         * as defined in the fmp_file. Don't rebuild for each one */
        for table in &self.file_handle.tables {
            let vmtable_tmp = VMTable {
                records: BTreeMap::new(),
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

pub struct Test {
    pub scripts: Vec<component::FMComponentScript>,
}


