use std::collections::HashMap;

use crate::component::FMComponentScript;
use crate::component::FMComponentTest;
use crate::file;
use crate::component;
use crate::fm_script_engine::fm_script_engine_instructions::Instruction;

pub struct Variable {
    name: String,
    value: String,
    global: bool,
}

impl Variable {
    pub fn new(n: String, val: String, g: bool) -> Self {
        Self {
            name: n,
            value: val,
            global: g,
        }
    } 

    pub fn set(&mut self, val: String) {
        self.value = val;
    }
}

pub struct VMTable {
    pub name: String,
    pub records: HashMap<String, Vec<String>>,
}

type Record = usize;

pub struct TestEnvironment<'a> {
    pub file_handle: &'a file::FmpFile,
    pub tables: Vec<VMTable>,
    /* Each table has it's own record pointer, as per FileMaker */
    pub record_ptrs: Vec<Option<Record>>,
    /* Each script has it's own instruction ptr.
     * on calling of a script, a new ptr is pushed.
     * When script is finished, we pop the instruction ptr.
     * Nothing more complex than function calls. No generators etc,
     * so this is fine.
     */
    pub instruction_ptr: Vec<(String, usize)>,
    pub variables: Vec<HashMap<String, Variable>>,
    pub current_test: Option<FMComponentTest>, 
}
impl<'a> TestEnvironment<'a> {

    pub fn new(file: &'a file::FmpFile) -> Self {
        Self {
            file_handle: file,
            tables: vec![],
            record_ptrs: vec![],
            instruction_ptr: vec![],
            variables: vec![],
            current_test: None,
        }
    }

    pub fn generate_test_environment(&mut self) {
        /* For each test, we will reuse the same table structure 
         * as defined in the fmp_file. Don't rebuild for each one */
        for table in &self.file_handle.tables {
            let vmtable_tmp = VMTable {
                name: table.1.table_name.clone(),
                records: HashMap::new(),
            };
            self.tables.push(vmtable_tmp);
            println!("Pushing Table \"{}\" to test environment", table.1.table_name);
            println!("Table has \"{}\" fields", table.1.fields.len());
            for f in &table.1.fields {
                println!("Pushing field: \"{}::{}\" to test environemnt", table.1.table_name, f.1.field_name);
                self.tables.last_mut().unwrap().records
                    .insert(f.1.field_name.to_string(), vec![]);
            }
            /* Each table is empty, therefore no pointer to a record */
            self.record_ptrs.push(None);
        }
    }

    pub fn run_tests(&mut self) {
        for test in &self.file_handle.tests {
            /* 1. Run the script 
             * 2. Check Assertions defined in test component
             * 3. Clean the test environment for next test */
            self.load_test(test.clone());
            while !self.instruction_ptr.is_empty() {
                self.step();
            }
        }
    }

    pub fn load_test(&mut self, test: FMComponentTest) {
        self.current_test = Some(test.clone());
        let script_name = &self.current_test.as_ref().unwrap().script.script_name;
        self.instruction_ptr.push((script_name.to_string(), 0));
        self.variables.push(HashMap::new());
    }

    pub fn step(&mut self) {

        assert!(self.current_test.is_some());
        let ip_handle: (String, usize);
        let mut script_handle: &FMComponentScript;
        let n_stack = self.instruction_ptr.len() - 1;
        ip_handle = self.instruction_ptr[n_stack].clone();
        if self.instruction_ptr.len() > 1 {
            script_handle = self.file_handle.scripts.get(&ip_handle.1).unwrap();
        } else {
            script_handle = &self.current_test.as_ref().unwrap().script;
        }
        
        if ip_handle.1 > script_handle.instructions.len() - 1{
            println!("Popping script: {}", ip_handle.0);
            self.instruction_ptr.pop();
            return;
        }
        let cur_instruction = &script_handle.instructions[ip_handle.1];
        match &cur_instruction.opcode {
            Instruction::SetVariable => {
                let name : &str = cur_instruction.switches[0].as_ref();
                let val : &str = cur_instruction.switches[1].as_ref();
                let tmp = Variable::new(name.to_string(), val.to_string(), false);
                let handle = self.variables[n_stack].get_mut(name);
                if handle.is_none() {
                    self.variables[n_stack].insert(name.to_string(), tmp);
                }
                self.instruction_ptr[n_stack].1 += 1;
            }
            _ => {
                eprintln!("Unimplemented instruction: {:?}", cur_instruction.opcode);
                self.instruction_ptr[n_stack].1 += 1;
            }
        }

    }


}


