
use crate::{compile::token::*, component::FMComponentTable, file::FmpFile, fm_script_engine::fm_script_engine_instructions::Instructions};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(ts: Vec<Token>) -> Self {
        Self {
            tokens: ts,
        }
    }
    pub fn parse_table(&self, tokens: &Vec<Token>, table: &mut FMComponentTable) {

    }

    pub fn parse_program(&self) -> FmpFile {

        let ret = FmpFile::new();

        for t in &self.tokens {
            match t.ttype {
                TokenType::Table => {
                    let table = FMComponentTable::new();
                    println!("TopLevel Table");
                }
                TokenType::Relationship => {
                    println!("TopLevel Relationship");
                }
                TokenType::ValueList => {
                    println!("TopLevel valuelist");
                }
                _ => {
                    println!("Unrecognized top level structure.");
                }
            }
        }

        return ret;

    }
}

