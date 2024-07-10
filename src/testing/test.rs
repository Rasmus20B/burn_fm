use std::collections::HashMap;
use std::io;
use std::io::Write;

use crate::component::FMComponentScript;
use crate::component::FMComponentTest;
use crate::file;
use crate::component;
use crate::fm_script_engine::fm_script_engine_instructions::Instruction;
use crate::testing::calc_eval;

use super::calc_eval::Node;
use super::calc_tokens;
use super::calc_tokens::Token;
use super::calc_tokens::TokenType;

pub struct Variable {
    pub name: String,
    pub value: String,
    pub global: bool,
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
    pub table_ptr: Option<usize>,
    pub loop_scopes: Vec<usize>,
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
            table_ptr: None,
            loop_scopes: vec![],
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
        self.table_ptr = Some(0);
    }

    pub fn run_tests(&mut self) {
        println!("Running test:");
        for test in &self.file_handle.tests {
            /* 1. Run the script 
             * 2. Check Assertions defined in test component
             * 3. Clean the test environment for next test */
            println!("Running test: {}", test.test_name);
            self.load_test(test.clone());
            while !self.instruction_ptr.is_empty() {
                self.step();
            }
            println!("Finished running test: {}", self.current_test.as_ref().unwrap().test_name);
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
        let mut ip_handle: (String, usize);
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
        let mut cur_instruction = &script_handle.instructions[ip_handle.1];
        match &cur_instruction.opcode {
            Instruction::SetVariable => {
                let name : &str = cur_instruction.switches[0].as_ref();
                let val : &str = &self.eval_calculation(&cur_instruction.switches[1]);
                println!("setting var {} to {}", name, val);
                let tmp = Variable::new(name.to_string(), val.to_string(), false);
                let handle = &mut self.variables[n_stack].get_mut(name);
                if handle.is_none() {
                    self.variables[n_stack].insert(name.to_string(), tmp);
                } else {
                    handle.as_mut().unwrap().value = tmp.value;
                }
                self.instruction_ptr[n_stack].1 += 1;
            },
            Instruction::SetField => {
                let name : &str = cur_instruction.switches[0].as_ref();
                let val : &str = &self.eval_calculation(&cur_instruction.switches[1]);
                let parts : Vec<&str> = val.split("::").collect();
                println!("Setting field {} to {}::{}", name, parts[0], parts[1]);
                self.instruction_ptr[n_stack].1 += 1;
            },
            Instruction::Loop => {
                println!("We in loop");
                self.loop_scopes.push(ip_handle.1);
                self.instruction_ptr[n_stack].1 += 1;
            },
            Instruction::EndLoop => {
                println!("end of the loop, back to the start");
                self.instruction_ptr[n_stack].1 = self.loop_scopes.last().unwrap() + 1;
                // self.instruction_ptr[n_stack].1 += 1;
            },
            Instruction::ExitLoopIf => {
                let val : &str = &self.eval_calculation(&cur_instruction.switches[0]);
                if val == "true" {
                    while cur_instruction.opcode != Instruction::EndLoop {
                        cur_instruction = &script_handle.instructions[ip_handle.1];
                        ip_handle.1 += 1;
                    }
                    self.instruction_ptr[n_stack].1 = ip_handle.1 + 1; 
                } else {
                    self.instruction_ptr[n_stack].1 += 1;
                }
            }
            Instruction::NewRecordRequest => {
                for (name, f) in &mut self.tables[self.table_ptr.unwrap()].records {
                    f.push(String::new());
                }
                println!("Creating a new record.");
                self.instruction_ptr[n_stack].1 += 1;
            },
            _ => {
                eprintln!("Unimplemented instruction: {:?}", cur_instruction.opcode);
                self.instruction_ptr[n_stack].1 += 1;
            }
        }
    }

    pub fn eval_calculation(&self, calculation: &str) -> String {
        let flush_buffer = |b: &str| -> Result<calc_tokens::Token, String> {
            match b {
                _ => {
                    let n = b.parse::<f64>();
                    if n.is_ok() {
                        Ok(Token::with_value(calc_tokens::TokenType::NumericLiteral, n.unwrap().to_string()))
                    } else if !b.as_bytes()[0].is_ascii_digit() {
                        Ok(Token::with_value(calc_tokens::TokenType::Identifier, b.to_string()))
                    } else {
                        Err("Invalid Identifier".to_string())
                    }
                }
            }
        };

        let mut tokens : Vec<Token> = vec![];
        let mut lex_iter = calculation.chars().into_iter().peekable();
        let mut buffer = String::new();
        while let Some(c) = &lex_iter.next() {
            if c.is_whitespace() && buffer.is_empty() {
                continue;
            }

            match c {
                ' ' => {
                    let b = flush_buffer(buffer.as_str());
                    buffer.clear();
                    tokens.push(b.unwrap());
                },
                '(' => {
                    let b = flush_buffer(buffer.as_str());
                    tokens.push(b.unwrap());
                    buffer.clear();
                    tokens.push(Ok::<calc_tokens::Token, String>(calc_tokens::Token::new(calc_tokens::TokenType::OpenParen)).unwrap());
                },
                '+' => {
                    let b = flush_buffer(buffer.as_str());
                    tokens.push(b.unwrap());
                    buffer.clear();
                    tokens.push(Ok::<calc_tokens::Token, String>(calc_tokens::Token::new(calc_tokens::TokenType::Plus)).unwrap());
                },
                '=' => {
                    let b = flush_buffer(buffer.as_str());
                    tokens.push(b.unwrap());
                    buffer.clear();
                    if *lex_iter.peek().unwrap() == '=' {
                        tokens.push(Ok::<calc_tokens::Token, String>(
                                calc_tokens::Token::new(calc_tokens::TokenType::Eq)).unwrap());
                        lex_iter.next();
                    }
                },
                _ => {
                    buffer.push(*c);
                }
            }
        }
        let b = flush_buffer(buffer.as_str());
        tokens.push(b.unwrap());

        /* Once we have our tokens, parse them into a binary expression. */
        
        let ast = calc_eval::Parser::new(tokens).parse().expect("unable to parse tokens.");
        self.evaluate(ast)
    }

    pub fn evaluate(&self, ast: Box<Node>) -> String {
        match *ast {
            Node::Unary { value, child } => {
                if child.is_none() {
                    return value;
                } else {
                    
                }
                "".to_string()
            },
            Node::Binary { left, operation, right } => {
                let lhs = self.evaluate(left);
                let rhs = self.evaluate(right);

                let mut lhs_n = lhs.parse::<f64>();
                let mut rhs_n = rhs.parse::<f64>();

                let scope = self.instruction_ptr.len() - 1;

                if lhs_n.is_err() {
                    lhs_n = Ok(self.variables[scope]
                               .get(&lhs)
                               .unwrap_or(&Variable::new(lhs, "0.0".to_string(), false))
                                    .value.parse::<f64>().expect("unable to parse variable value as number"));
                        // .get(&lhs).expect("Variable not in scope").value
                        // .clone().parse::<f64>().unwrap_or(0.0));
                }
                if rhs_n.is_err() {
                    rhs_n = Ok(self.variables[scope]
                               .get(&rhs)
                               .unwrap_or(&Variable::new(rhs, "0.0".to_string(), false))
                                    .value.parse::<f64>().expect("unable to parse variable value as number"));
                }

                match operation {
                    TokenType::Plus => { 
                        (lhs_n.clone().unwrap()
                         + 
                         rhs_n.clone().unwrap()
                         ).to_string() 
                    },
                    TokenType::Eq => { 
                        (lhs_n
                         == 
                         rhs_n
                         ).to_string() 
                    },
                    _ => { unreachable!()}
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::{compile::compiler::compile_burn, decompile::decompiler::decompile_fmp12_file};

    use super::TestEnvironment;

    #[test]
    pub fn basic_loop_test() {
        let code = "
        test BasicTest:
          script: [
            define blank_test() {
              set_variable(x, 1);
              loop {
                new_record_request();
                exit_loop_if(x == 10);
                set_variable(x, x + 1);
                set_field(Person::first_name, \"Kevin\")
              }
            }
          ],
          assertions:
            (assert_eq (empty? (Person)) (False)),
            (assert_eq (Person[0]) (Person (
              firstName: \"Kevin\",
              lastName: \"Matthews\",
              sex: \"male\",
              age: 50,
              jobid: null
            ))),
            ()
        end test;";
        let input = Path::new("tests/input/blank.fmp12");
        let mut file = decompile_fmp12_file(&input);
        let mut tests = compile_burn(code);
        file.tests.append(&mut tests.tests);
        let mut te : TestEnvironment = TestEnvironment::new(&file);
        te.generate_test_environment();
        te.run_tests();
        assert_eq!(te.tables[0].records.get("PrimaryKey").unwrap().len(), 10);
    }
}


