use std::collections::HashMap;

use crate::{component::FMComponentScript, fm_script_engine::fm_script_engine_instructions::Instruction};

use super::tokens::{self, Token, TokenType};


#[derive(Debug, PartialEq)]
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {

    pub fn new(toks: Vec<Token>) -> Self {
        Self {
            tokens: toks,
        }
    }

    pub fn parse(&self) -> Vec<FMComponentScript> {
        let mut scripts = vec![];
        let mut parser_iter = self.tokens.iter().peekable();

        while let Some(t) = parser_iter.next() {
            match t.ttype {
                TokenType::Define => {
                    let mut tmp = FMComponentScript::new();
                    let peeked = parser_iter.next().unwrap();
                    if peeked.ttype == TokenType::Identifier {
                        tmp.script_name = peeked.value.clone();
                    }

                    /* Parse arguments to script */
                    while let Some(t) = parser_iter.next() {
                        match t.ttype {
                            TokenType::Identifier => {
                                tmp.arguments.push(t.value.clone());
                            },
                            TokenType::Comma => {
                                continue;
                            },
                            TokenType::CloseParen => {
                                break;
                            },
                            _ => { eprintln!("invalid token."); }
                        };
                    }
                    scripts.push(tmp);
                },
                _ => {
                    eprintln!("Unrecognized token");
                }
            }
        }


        return scripts;
    }
}

#[cfg(test)]
mod tests {
    use crate::burn_script::lexer;

    use super::Parser;


    #[test]
    pub fn parse_test1() {
        let code = "
        define test_func(x, y) {
            set_variable(i, x);
            loop {
                exit_loop_if(i == y);
                set_variable(i, i + 1);
            }
            exit_script(i);
        }";
        let tokens = lexer::Lexer::new(code.to_string()).get_tokens();
        let scripts = Parser::new(tokens).parse();

        let handle = scripts[0].clone();
        assert_eq!(handle.script_name, "test_func");
        assert_eq!(handle.arguments, vec!["x", "y"]);
    }
}
