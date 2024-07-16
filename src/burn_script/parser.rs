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

    pub fn parse(&self) -> Result<Vec<FMComponentScript>, &str> {
        let mut scripts = vec![];
        let mut parser_iter = self.tokens.iter().peekable();

        let mut punc_stack: Vec<Instruction> = vec![];

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
                            TokenType::OpenParen => {
                                continue;
                            }
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
                        // println!("Found instruction: {:?} : {}", t.ttype, t.value);
                        match t.ttype {
                            TokenType::Identifier => {
                                if let Ok(op) = Instruction::from_str(&t.value) {
                                    let mut step = ScriptStep {
                                        opcode: op,
                                        index: 0,
                                        switches: vec![],
                                    };
                                    let mut buffer = String::new();
                                    while let Some(t) = parser_iter.next() {
                                        match t.ttype {
                                            TokenType::Identifier | TokenType::NumericLiteral => {
                                                buffer.push_str(&t.value);
                                            }
                                            TokenType::SemiColon => {
                                                break;
                                            },
                                            TokenType::OpenBracket => {
                                                break;
                                            }
                                            TokenType::OpenParen => {
                                                continue;
                                            },
                                            TokenType::CloseParen => {
                                                step.switches.push(buffer.clone());
                                                buffer.clear();
                                                continue;
                                            }
                                            TokenType::Comma => {
                                                step.switches.push(buffer.clone());
                                                buffer.clear();
                                                continue;
                                            },
                                            TokenType::Eq => {
                                                buffer.push_str("==");
                                            },
                                            TokenType::Neq => {
                                                buffer.push_str("!=");
                                            },
                                            TokenType::Plus => {
                                                buffer.push('+');
                                            },
                                            _ => { eprintln!("Unexpected Token."); }
                                        }
                                    }
                                    tmp.instructions.push(step);
                                } else {
                                    eprintln!("Invalid script step: {}", t.value);
                                }
                            },

                            TokenType::CloseBracket => {
                                let top = punc_stack.last();
                                if top.is_none() {
                                    break;
                                }

                                match parser_iter.peek().unwrap().ttype {
                                    TokenType::Else => {
                                        continue;
                                    }
                                    _ => {}
                                }
                                let op: Instruction;
                                match top.unwrap() {
                                    Instruction::If => {
                                        op = Instruction::EndIf;
                                    },
                                    Instruction::Loop => {
                                        op = Instruction::EndLoop;
                                    },
                                    _ => {
                                        return Err("Invalid scope signifier.");
                                    }
                                }
                                punc_stack.pop();
                                let step = ScriptStep {
                                    opcode: op,
                                    index: 0,
                                    switches: vec![],
                                };
                                tmp.instructions.push(step);
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
                                punc_stack.push(Instruction::Loop);
                            },
                            TokenType::If => {
                                let mut step = ScriptStep {
                                    opcode: Instruction::If,
                                    index: 0,
                                    switches: vec![]
                                };
                                let mut buf = "".to_string();
                                while let Some(n) = parser_iter.next() {
                                    match n.ttype {
                                        TokenType::CloseParen => {
                                            step.switches.push(buf.clone());
                                            break;
                                        },
                                        TokenType::Identifier | TokenType::NumericLiteral => {
                                            buf.push_str(&n.value);
                                        }
                                        TokenType::OpenParen => {
                                            continue;
                                        }
                                        TokenType::Eq => {
                                            buf.push_str("==");
                                        },
                                        TokenType::Neq => {
                                            buf.push_str("!=");
                                        },
                                        TokenType::Plus => {
                                            buf.push('+');
                                        },
                                        _ => {
                                            buf.push_str(&t.value);
                                        },
                                    }
                                    
                                }
                                tmp.instructions.push(step);
                                punc_stack.push(Instruction::If);
                            },
                            TokenType::Else => {
                                let check = parser_iter.next();
                                if check.unwrap().ttype != TokenType::OpenBracket {
                                    eprintln!("Expected '{{' after loop.");
                                    break;
                                }
                                let step = ScriptStep {
                                    opcode: Instruction::Else,
                                    index: 0,
                                    switches: vec![]
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
        return Ok(scripts);
    }
}

#[cfg(test)]
mod tests {
    use crate::{burn_script::lexer, fm_script_engine::fm_script_engine_instructions::{Instruction, ScriptStep}};

    use super::Parser;


    #[test]
    pub fn parse_test_basic() {
        let code = "
        define test_func(x, y) {
            set_variable(i, x);
            loop {
                exit_loop_if(i == y);
                set_variable(i, i + 1);
                if(i == 7) {
                    set_variable(x, 20);
                } else {
                    set_variable(x, 50);
                }
            }
            exit_script(i);
        }";
        let tokens = lexer::Lexer::new(code.to_string()).get_tokens();
        let scripts = Parser::new(tokens).parse().expect("Unable to parse token stream");

        let handle = scripts[0].clone();
        assert_eq!(handle.script_name, "test_func");
        assert_eq!(handle.arguments, vec!["x", "y"]);

        let steps_actual = vec![
            ScriptStep { opcode: Instruction::SetVariable,
                         switches: vec!["i".to_string(), "x".to_string()],
                         index: 0,
            },
            ScriptStep { opcode: Instruction::Loop,
                         switches: vec![],
                         index: 0,
            },
            ScriptStep { opcode: Instruction::ExitLoopIf,
                         switches: vec!["i==y".to_string()],
                         index: 0,
            },
            ScriptStep { opcode: Instruction::SetVariable,
                         switches: vec!["i".to_string(), "i+1".to_string()],
                         index: 0,
            },
            ScriptStep { opcode: Instruction::If,
                         switches: vec!["i==7".to_string()],
                         index: 0,
            },
            ScriptStep { opcode: Instruction::SetVariable,
                         switches: vec!["x".to_string(), "20".to_string()],
                         index: 0,
            },
            ScriptStep { opcode: Instruction::Else,
                         switches: vec![],
                         index: 0,
            },
            ScriptStep { opcode: Instruction::SetVariable,
                         switches: vec!["x".to_string(), "50".to_string()],
                         index: 0,
            },
            ScriptStep { opcode: Instruction::EndIf,
                         switches: vec![],
                         index: 0,
            },
            ScriptStep { opcode: Instruction::EndLoop,
                         switches: vec![],
                         index: 0,
            },
            ScriptStep { opcode: Instruction::ExitScript,
                         switches: vec!["i".to_string()],
                         index: 0,
            },
        ];
        println!("{:?}", handle.instructions);
        for (i, step) in steps_actual.iter().enumerate() {
            assert_eq!(*step, handle.instructions[i]);
        }
    }
}
