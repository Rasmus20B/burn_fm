use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::Deref;

use color_print::cprintln;
use crate::component::FMComponentScript;
use crate::component::FMComponentTest;
use crate::file;
use crate::fm_script_engine::fm_script_engine_instructions::Instruction;
use crate::fm_script_engine::fm_script_engine_instructions::ScriptStep;
use crate::testing::calc_eval;

use super::calc_tokens;

#[derive(Debug)]
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
}

pub struct VMTable {
    pub name: String,
    pub records: HashMap<String, Vec<String>>,
}

#[derive(PartialEq, Debug)]
pub enum TestState {
    Pass,
    Fail
}

#[derive(PartialEq, Debug)]
enum OperandType {
    Number,
    Text,
    FieldName,
}

#[derive(PartialEq, Debug)]
struct Operand<'a> {
    value: &'a str,
    otype: OperandType
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
    pub test_state: TestState,

    pub punc_stack: Vec<Instruction>,
    pub branch_taken: bool,
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
            test_state: TestState::Pass,
            punc_stack: vec![],
            branch_taken: false,
        }
    }

    pub fn generate_test_environment(&mut self) {
        /* For each test, we will reuse the same table structure 
         * as defined in the fmp_file. Don't rebuild for each one */
        self.record_ptrs.resize(self.file_handle.tables.len(), None);
        for (i, table) in self.file_handle.tables.clone().into_iter().enumerate() {
            let vmtable_tmp = VMTable {
                name: table.1.table_name.clone(),
                records: HashMap::new(),
            };
            self.tables.push(vmtable_tmp);
            // println!("Pushing Table \"{}\" to test environment", table.1.table_name);
            // println!("Table has \"{}\" fields", table.1.fields.len());
            for f in &table.1.fields {
                // println!("Pushing field: \"{}::{}\" to test environemnt", table.1.table_name, f.1.field_name);
                self.tables.last_mut().unwrap().records
                    .insert(f.1.field_name.to_string(), vec![]);
            }
            /* Each table is empty, therefore no pointer to a record */
            if table.0 == 1 {
                self.table_ptr = Some(i);
            }
        }
    }

    #[allow(unused)]
    pub fn run_tests(&mut self) {
        for test in &self.file_handle.tests {
            /* 1. Run the script 
             * 2. Check Assertions defined in test component
             * 3. Clean the test environment for next test */
            println!("Running test: {}", test.test_name);
            self.load_test(test.clone());
            while !self.instruction_ptr.is_empty() {
                self.step();
            }
            if self.test_state == TestState::Pass {
                cprintln!("Test {} outcome: <green>Success</green>", self.current_test.as_ref().unwrap().test_name);
            } else if self.test_state == TestState::Fail {
                cprintln!("Test {} outcome: <red>Fail</red>", self.current_test.as_ref().unwrap().test_name);
            }

        }
    }
    pub fn run_tests_with_cleanup(&mut self) {
        for test in &self.file_handle.tests {
            /* 1. Run the script 
             * 2. Check Assertions defined in test component
             * 3. Clean the test environment for next test */
            println!("Running test: {}", test.test_name);
            self.load_test(test.clone());
            while self.test_state == TestState::Pass && !self.instruction_ptr.is_empty() {
                self.step();
            }
            if self.test_state == TestState::Pass {
                cprintln!("Test {} outcome: <green>Success</green>", self.current_test.as_ref().unwrap().test_name);
            } else if self.test_state == TestState::Fail {
                cprintln!("Test {} outcome: <red>Fail</red>", self.current_test.as_ref().unwrap().test_name);
            }
            for t in &mut self.tables {
                t.records.clear();
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
        let mut ip_handle: (String, usize);
        let mut script_handle = &FMComponentScript::new();
        let n_stack = self.instruction_ptr.len() - 1;
        ip_handle = self.instruction_ptr[n_stack].clone();
        let s_name = self.instruction_ptr[n_stack].0.clone();
        if self.instruction_ptr.len() > 1 {
            for s in &self.file_handle.scripts {
                if s.1.script_name == s_name {
                    script_handle = s.1;
                    break;
                }
            }
        } else {
            script_handle = &self.current_test.as_ref().unwrap().script;
        }

        if script_handle.instructions.is_empty() ||
            ip_handle.1 > script_handle.instructions
                .clone().into_iter().map(|x| x.0).max().unwrap() {
                println!("Popping script: {}", ip_handle.0);
                self.instruction_ptr.pop();
                return;
        }

        let mut cur_instruction = &script_handle.instructions[&ip_handle.1];
        match &cur_instruction.opcode {
            Instruction::PerformScript => {
                let script_name = self.eval_calculation(&cur_instruction.switches[0])
                    .strip_suffix('"').unwrap()
                    .strip_prefix('"').unwrap().to_string();
                self.variables.push(HashMap::new());

                for s in &self.file_handle.scripts {
                    if s.1.script_name == script_name {
                        self.instruction_ptr[n_stack].1 += 1;
                        self.instruction_ptr.push((script_name.clone(), *s.1.instructions.keys().take(1).collect::<Vec<_>>()[0]));
                        println!("calling {}", script_name);
                        break;
                    }
                }
            },
            Instruction::SetVariable => {
                let name : &str = cur_instruction.switches[0].as_ref();
                let val : &str = &self.eval_calculation(&cur_instruction.switches[1]);
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

                let parts : Vec<&str> = name.split("::").collect();
                let mut table_handle : Option<&mut VMTable> = None;
                let mut n = 0;
                for (i, table) in self.tables.iter_mut().enumerate() {
                    if table.name == parts[0] {
                        table_handle = Some(table);
                        n = i;
                        break;
                    }
                }

                if table_handle.is_none() {
                    eprintln!("Table does not exist.");
                    self.instruction_ptr[n_stack].1 += 1;
                    return;
                }

                table_handle.unwrap().records.get_mut(parts[1])
                    .expect("Field does not exist.")[self.record_ptrs[n].unwrap()] = val.to_string();
                self.instruction_ptr[n_stack].1 += 1;
            },
            Instruction::Loop => {
                self.loop_scopes.push(ip_handle.1);
                self.punc_stack.push(Instruction::Loop);
                self.instruction_ptr[n_stack].1 += 1;
            },
            Instruction::If => {
                let val = &self.eval_calculation(&cur_instruction.switches[0]);
                if val == "true" {
                    self.instruction_ptr[n_stack].1 += 1;
                    self.branch_taken = true;
                } else {
                    while self.instruction_ptr[n_stack].1 < script_handle.instructions.len() {
                        cur_instruction = &script_handle.instructions[&self.instruction_ptr[n_stack].1];
                        match cur_instruction.opcode {
                            Instruction::EndIf => {
                                return;
                            },
                            Instruction::Else => {
                                return;
                            },
                            Instruction::ElseIf => {
                                return;
                            },
                            _ => {}
                        }
                        self.instruction_ptr[n_stack].1 += 1;
                    }
                }
            },
            Instruction::ElseIf => {
                if self.branch_taken == true {
                    while self.instruction_ptr[n_stack].1 < script_handle.instructions.len() {
                        cur_instruction = &script_handle.instructions[&self.instruction_ptr[n_stack].1];
                        match cur_instruction.opcode {
                            Instruction::EndIf => {
                                self.branch_taken = false;
                                return;
                            },
                            _ => {self.instruction_ptr[n_stack].1 += 1;}
                        }
                    }
                }
                let val = &self.eval_calculation(&cur_instruction.switches[0]);
                if val == "true" {
                    self.instruction_ptr[n_stack].1 += 1;
                    self.branch_taken = true;
                } else {
                    self.instruction_ptr[n_stack].1 += 1;
                    while self.instruction_ptr[n_stack].1 < script_handle.instructions.len() {
                        cur_instruction = &script_handle.instructions[&self.instruction_ptr[n_stack].1];
                        match cur_instruction.opcode {
                            Instruction::EndIf => {
                                return;
                            },
                            Instruction::Else => {
                                return;
                            },
                            Instruction::ElseIf => {
                                return;
                            },
                            _ => {self.instruction_ptr[n_stack].1 += 1;}
                        }
                    }
                }
            }
            Instruction::Else => {
                if self.branch_taken == true {
                    while self.instruction_ptr[n_stack].1 < script_handle.instructions.len() {
                        cur_instruction = &script_handle.instructions[&self.instruction_ptr[n_stack].1];
                        match cur_instruction.opcode {
                            Instruction::EndIf => {
                                self.branch_taken = false;
                                return;
                            },
                            _ => {self.instruction_ptr[n_stack].1 += 1;}
                        }
                    }
                }
                self.instruction_ptr[n_stack].1 += 1;
            }

            Instruction::EndIf => {
                self.branch_taken = false;
                self.instruction_ptr[n_stack].1 += 1;
            }
            Instruction::EndLoop => {
                if *self.punc_stack.last().unwrap_or(&Instruction::EndLoop) != Instruction::Loop {
                    eprintln!("invalid scope resultion. Please check that loop and if blocks are terminated correctly.");
                }
                self.instruction_ptr[n_stack].1 = self.loop_scopes.last().unwrap() + 1;
            },
            Instruction::ExitLoopIf => {
                let val : &str = &self.eval_calculation(&cur_instruction.switches[0]);
                if val == "true" {
                    while cur_instruction.opcode != Instruction::EndLoop {
                        cur_instruction = &script_handle.instructions[&ip_handle.1];
                        ip_handle.1 += 1;
                    }
                    self.instruction_ptr[n_stack].1 = ip_handle.1; 
                } else {
                    self.instruction_ptr[n_stack].1 += 1;
                }
            }
            Instruction::NewRecordRequest => {
                for (name, f) in &mut self.tables[self.table_ptr.unwrap()].records {
                    f.push(String::new());
                }
                let handle = &mut self.record_ptrs[self.table_ptr.unwrap()];
                if handle.is_none() {
                    *handle = Some(0);
                } else {
                    *handle = Some(handle.unwrap() + 1);
                }
                self.instruction_ptr[n_stack].1 += 1;
            },
            Instruction::ShowCustomDialog => {
                println!("{}", &self.eval_calculation(&cur_instruction.switches[0]));
                self.instruction_ptr[n_stack].1 += 1;
            },
            Instruction::Assert => {
                let val : &str = &self.eval_calculation(&cur_instruction.switches[0]);
                if val == "false" {
                    cprintln!("<red>Assertion failed<red>: {}", cur_instruction.switches[0]);
                    self.test_state = TestState::Fail;
                } 
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
                        Ok(calc_tokens::Token::with_value(calc_tokens::TokenType::NumericLiteral, n.unwrap().to_string()))
                    } else if !b.as_bytes()[0].is_ascii_digit() {
                        Ok(calc_tokens::Token::with_value(calc_tokens::TokenType::Identifier, b.to_string()))
                    } else {
                        Err("Invalid Identifier".to_string())
                    }
                }
            }
        };

        let mut tokens : Vec<calc_tokens::Token> = vec![];
        let mut lex_iter = calculation.chars().into_iter().peekable();
        let mut buffer = String::new();
        while let Some(c) = &lex_iter.next() {
            if c.is_whitespace() && buffer.is_empty() {
                continue;
            }

            match c {
                ' ' => {
                    if buffer.len() > 0 {
                        let b = flush_buffer(buffer.as_str());
                        buffer.clear();
                        tokens.push(b.unwrap());
                    }
                },
                '(' => {
                    if buffer.len() > 0 {
                        let b = flush_buffer(buffer.as_str());
                        buffer.clear();
                        tokens.push(b.unwrap());
                    }
                    tokens.push(Ok::<calc_tokens::Token, String>(calc_tokens::Token::new(calc_tokens::TokenType::OpenParen)).unwrap());
                },
                '+' => {
                    if buffer.len() > 0 {
                        let b = flush_buffer(buffer.as_str());
                        buffer.clear();
                        tokens.push(b.unwrap());
                    }
                    tokens.push(Ok::<calc_tokens::Token, String>(calc_tokens::Token::new(calc_tokens::TokenType::Plus)).unwrap());
                },
                '&' => {
                    if buffer.len() > 0 {
                        let b = flush_buffer(buffer.as_str());
                        buffer.clear();
                        tokens.push(b.unwrap());
                    }
                    tokens.push(Ok::<calc_tokens::Token, String>(calc_tokens::Token::new(calc_tokens::TokenType::Ampersand)).unwrap());
                },
                '!' => {
                    if buffer.len() > 0 {
                        let b = flush_buffer(buffer.as_str());
                        buffer.clear();
                        tokens.push(b.unwrap());
                    }
                    if *lex_iter.peek().unwrap() == '=' {
                        tokens.push(Ok::<calc_tokens::Token, String>(
                                calc_tokens::Token::new(calc_tokens::TokenType::Neq)).unwrap());
                        lex_iter.next();
                    }
                },
                '=' => {
                    if buffer.len() > 0 {
                        let b = flush_buffer(buffer.as_str());
                        buffer.clear();
                        tokens.push(b.unwrap());
                    }
                    if *lex_iter.peek().unwrap() == '=' {
                        tokens.push(Ok::<calc_tokens::Token, String>(
                                calc_tokens::Token::new(calc_tokens::TokenType::Eq)).unwrap());
                    }
                },
                '<' => {
                    if buffer.len() > 0 {
                        let b = flush_buffer(buffer.as_str());
                        buffer.clear();
                        tokens.push(b.unwrap());
                    }
                    if *lex_iter.peek().unwrap() == '=' {
                        tokens.push(Ok::<calc_tokens::Token, String>(
                                calc_tokens::Token::new(calc_tokens::TokenType::Ltq)).unwrap());
                    } else {
                        tokens.push(Ok::<calc_tokens::Token, String>(
                                calc_tokens::Token::new(calc_tokens::TokenType::Lt)).unwrap());
                    }
                },
                '>' => {
                    if buffer.len() > 0 {
                        let b = flush_buffer(buffer.as_str());
                        buffer.clear();
                        tokens.push(b.unwrap());
                    }
                    if *lex_iter.peek().unwrap() == '=' {
                        tokens.push(Ok::<calc_tokens::Token, String>(
                                calc_tokens::Token::new(calc_tokens::TokenType::Gtq)).unwrap());
                    } else {
                        tokens.push(Ok::<calc_tokens::Token, String>(
                                calc_tokens::Token::new(calc_tokens::TokenType::Gt)).unwrap());
                    }
                }
                '"' => {
                    if buffer.len() > 0 {
                        let b = flush_buffer(buffer.as_str());
                        buffer.clear();
                        tokens.push(b.unwrap());
                    }
                    buffer.push(*c);
                    while let Some(c) = &lex_iter.next() {
                        if *c == '"' {
                            buffer.push(*c);
                            break;
                        }
                        buffer.push(*c);
                    }
                    tokens.push(calc_tokens::Token::with_value(calc_tokens::TokenType::String, buffer.clone()));
                    buffer.clear();
                },
                ':' => {
                    buffer.push(*c);
                    if buffer.is_empty() || *lex_iter.peek().unwrap_or(&'?') != ':' { 
                        eprintln!("invalid ':' found.");
                        buffer.push(*c);
                    }
                    lex_iter.next();
                    buffer.push(*c);
                    while let Some(c) = &lex_iter.next() {
                        buffer.push(*c);
                        if !lex_iter.peek().unwrap().is_alphanumeric() {
                            break;
                        }
                    }
                    tokens.push(calc_tokens::Token::with_value(calc_tokens::TokenType::Identifier, buffer.clone()));
                    buffer.clear();
                },
                _ => {
                    buffer.push(*c);
                }
            }
        }

        if buffer.len() > 0 {
            let b = flush_buffer(buffer.as_str());
            buffer.clear();
            tokens.push(b.unwrap());
        }
        /* Once we have our tokens, parse them into a binary expression. */
        let ast = calc_eval::Parser::new(tokens).parse().expect("unable to parse tokens.");
        self.evaluate(ast)
    }

    fn get_operand_val(&'a self, val: &'a str) -> Operand {
        let r = val.parse::<i64>();
        if r.is_ok() {
            return Operand {
                otype: OperandType::Number,
                value: val
            }
        }

        if val.starts_with("\"") && val.ends_with("\"") {
            return Operand {
                otype: OperandType::Text,
                value: val
            }
        }

        let fieldname = val.split("::").collect::<Vec<&str>>();
        if fieldname.len() == 2 {
            let ts = &self.tables.iter()
                .filter(|x| x.name == fieldname[0])
                .collect::<Vec<_>>();
            if ts.is_empty() { 
                eprintln!("Unknown table: {}", fieldname[0]); 
                return Operand {
                    otype: OperandType::Text,
                    value: val
                }
            }
            let t_ptr = ts[0];

            let f_ptr = t_ptr.records.keys()
                .zip(0..t_ptr.records.keys().len())
                .filter(|x| x.0 == fieldname[1])
                .collect::<Vec<_>>();


            let mut n = 0;
            let mut table_handle : Option<&VMTable> = None;
            for (i, table) in self.tables.iter().enumerate() {
                if table.name == fieldname[0] {
                    table_handle = Some(table);
                    n = i;
                    break;
                }
            }

            if table_handle.is_none() {
                eprintln!("Table does not exist.");
                return Operand {
                    otype: OperandType::Text,
                    value: ""
                };
            }

            // println!("{:?} -> {:?}", 
            //     fieldname[1], 
            //     &table_handle.unwrap().records.get(fieldname[1]).unwrap()[self.record_ptrs[n].unwrap()]);
            return self.get_operand_val(&table_handle.unwrap().records.get(fieldname[1]).unwrap()[self.record_ptrs[n].unwrap()]);
        } else {
            let scope = self.instruction_ptr.len() - 1;
            let var_val = self.variables[scope]
                .get(val);

            if var_val.is_none() {
                eprintln!("Unknown variable: {}", val); 
                return Operand {
                    otype: OperandType::Text,
                    value: ""
                }
            }
            return self.get_operand_val(&var_val.unwrap().value);
        }

        return Operand {
            otype: OperandType::Text,
            value: ""
        }

    }

    pub fn evaluate(&self, ast: Box<calc_eval::Node>) -> String {

        match *ast {
            calc_eval::Node::Unary { value, child } => {
                if child.is_none() {
                    return self.get_operand_val(value.as_str())
                        .value.to_string();
                } else {
                }
                "".to_string()
            },
            calc_eval::Node::Binary { left, operation, right } => {
                let lhs_wrap = &self.evaluate(left);
                let rhs_wrap = &self.evaluate(right);
                let lhs = self.get_operand_val(lhs_wrap);
                let rhs = self.get_operand_val(rhs_wrap);


                match operation {
                    calc_tokens::TokenType::Plus => { 
                        if lhs.otype != OperandType::Number
                            || rhs.otype != OperandType::Number {
                                eprintln!("Unable to add non-number types.");
                                return "undefined".to_string();
                        }
                        (lhs.value.parse::<f64>().unwrap()
                         + 
                         rhs.value.parse::<f64>().unwrap()
                         ).to_string() 
                    },
                    calc_tokens::TokenType::Eq => { 
                        // println!("lhs: {:?}, rhs: {:?} == {:?}", lhs, rhs, lhs == rhs);
                        (lhs.value
                         == 
                         rhs.value
                         ).to_string() 
                    },
                    calc_tokens::TokenType::Neq => { 
                        (lhs.value
                         != 
                         rhs.value
                         ).to_string() 
                    },
                    calc_tokens::TokenType::Lt => {
                        (lhs.value.parse::<f64>().unwrap()
                         <
                         rhs.value.parse::<f64>().unwrap())
                         .to_string()
                    },
                    calc_tokens::TokenType::Ltq => {
                        (lhs.value.parse::<f64>().unwrap()
                         <=
                         rhs.value.parse::<f64>().unwrap())
                         .to_string()
                    },
                    calc_tokens::TokenType::Gt => {
                        (lhs.value.parse::<f64>().unwrap()
                         >
                         rhs.value.parse::<f64>().unwrap())
                         .to_string()
                    },
                    calc_tokens::TokenType::Gtq => {
                        (lhs.value.parse::<f64>().unwrap()
                         >=
                         rhs.value.parse::<f64>().unwrap())
                         .to_string()
                    },
                    calc_tokens::TokenType::Ampersand => { 
                        let lhs = lhs.value.replace('"', "");
                        let rhs = rhs.value.replace('"', "");
                        format!("\"{lhs}{rhs}\"")
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
              set_variable(x, 0);
              loop {
                new_record_request();
                exit_loop_if(x == 9);
                assert(x != 10);
                if(x == 7) {
                    set_field(blank::PrimaryKey, \"Kevin\");
                } elif(x == 1) {
                    set_field(blank::PrimaryKey, \"alvin\" & \" Presley\");
                } elif(x == 2) {
                    set_field(blank::PrimaryKey, \"NAHHH\");
                } else {
                    set_field(blank::PrimaryKey, \"Jeff\" & \" Keighly\");
                }
                set_variable(x, x + 1);
              }
            }
          ],
        end test;";
        let input = Path::new("tests/input/blank.fmp12");
        let mut file = decompile_fmp12_file(&input);
        let mut tests = compile_burn(code);
        file.tests.append(&mut tests.tests);
        let mut te : TestEnvironment = TestEnvironment::new(&file);
        te.generate_test_environment();
        te.run_tests();
        assert_eq!(te.tables[te.table_ptr.unwrap()].records.get("PrimaryKey").unwrap().len(), 10);
        assert_eq!(te.tables[te.table_ptr.unwrap()].records.get("PrimaryKey").unwrap()[0], "\"Jeff Keighly\"");
        assert_eq!(te.tables[te.table_ptr.unwrap()].records.get("PrimaryKey").unwrap()[1], "\"alvin Presley\"");
        assert_eq!(te.tables[te.table_ptr.unwrap()].records.get("PrimaryKey").unwrap()[2], "\"NAHHH\"");
        assert_eq!(te.tables[te.table_ptr.unwrap()].records.get("PrimaryKey").unwrap()[3], "\"Jeff Keighly\"");
        assert_eq!(te.tables[te.table_ptr.unwrap()].records.get("PrimaryKey").unwrap()[4], "\"Jeff Keighly\"");
        assert_eq!(te.tables[te.table_ptr.unwrap()].records.get("PrimaryKey").unwrap()[5], "\"Jeff Keighly\"");
        assert_eq!(te.tables[te.table_ptr.unwrap()].records.get("PrimaryKey").unwrap()[6], "\"Jeff Keighly\"");
        assert_eq!(te.tables[te.table_ptr.unwrap()].records.get("PrimaryKey").unwrap()[7], "\"Kevin\"");
        assert_eq!(te.tables[te.table_ptr.unwrap()].records.get("PrimaryKey").unwrap()[8], "\"Jeff Keighly\"");
    }
}

