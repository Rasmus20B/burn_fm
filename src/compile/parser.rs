
use crate::{compile::token::*, script_engine::script_engine_instructions::Instructions};

pub fn parse_program(tokens: &Vec<Token>) {

    for t in tokens {
        match t.ttype {
            TokenType::Table => {
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

}
