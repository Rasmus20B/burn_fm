use std::{collections::HashMap, str::FromStr};

use clap::parser;

use crate::{component::FMComponentScript, fm_script_engine::fm_script_engine_instructions::{Instruction, ScriptStep}};

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
                                continue;
                            },
                            TokenType::OpenBracket => {
                                break;
                            }
                            _ => { eprintln!("invalid token."); }
                        };
                    }

                    /* Parse the instructions inside the script with their options */
                    while let Some(t) = parser_iter.next() {
                        println!("Found instruction: {:?} : {}", t.ttype, t.value);
                        match t.ttype {
                            TokenType::Identifier => {
                                if let Ok(op) = Instruction::from_str(&t.value) {
                                    let mut step = ScriptStep {
                                        opcode: op,
                                        index: 0,
                                        switches: vec![],
                                    };

                                    while let Some(t) = parser_iter.next() {
                                        match t.ttype {
                                            TokenType::Identifier => {
                                                step.switches.push(t.value.clone());
                                            }
                                            TokenType::SemiColon => {
                                                println!("Breaking from current");
                                                break;
                                            },
                                            TokenType::OpenParen => {
                                                continue;
                                            },
                                            TokenType::CloseParen => {
                                                continue;
                                            }
                                            TokenType::Comma => {
                                                continue;
                                            }
                                            _ => { eprintln!("Unexpected Token."); }
                                        }
                                    }
                                    tmp.instructions.push(step);
                                } else {
                                    eprintln!("Invalid script step: {}", t.value);
                                }
                            },
                            TokenType::Loop => {
                                let check = parser_iter.next();
                                if check.unwrap().ttype != TokenType::OpenBracket {
                                    eprintln!("Expected '{{' after loop.");
                                    break;
                                }
                                let step = ScriptStep {
                                    opcode: Instruction::Loop,
                                    index: 0,
                                    switches: vec![],
                                };
                                tmp.instructions.push(step);
                            }
                            _ => { eprintln!("Invalid token in script"); }
                        }
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
    use crate::{burn_script::lexer, fm_script_engine::fm_script_engine_instructions::Instruction};

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

        let instrs_actual = vec![Instruction::SetVariable,
                                 Instruction::Loop,
                                 Instruction::ExitLoopIf,
                                 Instruction::SetVariable,
                                 Instruction::ExitScript];
        for (i, step) in instrs_actual.iter().enumerate() {
            assert_eq!(*step, handle.instructions[i].opcode);
        }
    }
}
