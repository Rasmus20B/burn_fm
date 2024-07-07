
use crate::{compile::token::*, component::FMComponentTable, file::FmpFile, fm_script_engine::fm_script_engine_instructions::ScriptStep};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(ts: Vec<Token>) -> Self {
        Self {
            tokens: ts,
        }
    }
    pub fn parse_table(&self, table: &mut FMComponentTable) {

    }

    pub fn parse_program(&self) -> FmpFile {

        let mut ret = FmpFile::new();
        let mut parser_iter = self.tokens.iter().peekable().clone();

        while let Some(t) = parser_iter.next() {
            match t.ttype {
                TokenType::Table => {
                    let mut table = FMComponentTable::new();
                    if parser_iter.peek().unwrap().ttype != TokenType::Identifier {
                        eprintln!("Expected Table name after \"table\" keyword.");
                    } else {
                        println!("table: {}", parser_iter.peek().unwrap().text);
                        table.table_name = parser_iter.next().unwrap().text.clone();
                    }
                    while let Some(mut n) = parser_iter.next() {
                        match n.ttype {
                            TokenType::End => 
                            {
                                n = parser_iter.next().unwrap();
                                if n.ttype == TokenType::Table {
                                    break;
                                } else {
                                    eprintln!("Unexpected {} after \"end\"", n.text);
                                }
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                    ret.tables.insert(ret.tables.len(), table);
                },
                TokenType::Relationship => {
                    println!("TopLevel Relationship");
                },
                TokenType::ValueList => {
                    println!("TopLevel valuelist");
                },
                TokenType::Script => {
                    println!("TopLevel Script");
                },
                TokenType::TableOccurence => {
                    println!("TopLevel Table Occurence");
                },
                TokenType::Test => {
                    println!("TopLevel Test");
                },
                _ => {
                    println!("Unrecognized top level structure.");
                }
            }
        }

        return ret;

    }
}

