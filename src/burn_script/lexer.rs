
use crate::burn_script::tokens::*;

#[derive(Debug, PartialEq)]
pub struct Lexer {
    code: String,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Self { code: text }
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        let mut result = vec![];
        let mut buffer = String::new();
        let mut lex_iter = self.code.chars().into_iter().enumerate().peekable();
        while let Some((idx, c)) = &lex_iter.next() {
            if buffer.is_empty() && c.is_whitespace() {
                continue;
            }
            let b = buffer.as_str();

            let tmp: Option<Vec<_>> = match c {
                x if x.is_whitespace() =>  
                {
                    let mut ret: Vec<Token> = vec![];
                    if !buffer.is_empty() {
                        match b {
                            "define" => { ret.push(Token::new(TokenType::Define)); }
                            "let" => { ret.push(Token::new(TokenType::Let)); }
                            "loop" => { ret.push(Token::new(TokenType::Loop)); }
                            _ => {
                                ret.push(Token::with_value(TokenType::Identifier, &buffer));
                            }
                        }
                    }
                    Some(ret)
                },
                '(' =>
                {
                    let mut ret: Vec<Token> = vec![];
                    if !buffer.is_empty() {
                        ret.push(Token::with_value(TokenType::Identifier, &buffer));
                    }
                    ret.push(Token::new(TokenType::OpenParen));
                    Some(ret)
                },
                '{' =>
                {
                    let mut ret: Vec<Token> = vec![];
                    if !buffer.is_empty() {
                        ret.push(Token::with_value(TokenType::Identifier, &buffer));
                    }
                    ret.push(Token::new(TokenType::OpenBracket));
                    Some(ret)
                },
                '}' =>
                {
                    let mut ret: Vec<Token> = vec![];
                    if !buffer.is_empty() {
                        ret.push(Token::with_value(TokenType::Identifier, &buffer));
                    }
                    ret.push(Token::new(TokenType::CloseBracket));
                    Some(ret)
                },
                ')' =>
                {
                    let mut ret: Vec<Token> = vec![];
                    if !buffer.is_empty() {
                        let n = buffer.parse::<f64>();
                        if n.is_ok() {
                            ret.push(Token::with_value(TokenType::NumericLiteral, &buffer))
                        } else {
                            ret.push(Token::with_value(TokenType::Identifier, &buffer))
                        }
                    }
                    ret.push(Token::new(TokenType::CloseParen));
                    Some(ret)
                },
                ',' => 
                {

                    let mut ret: Vec<Token> = vec![];

                    if !buffer.is_empty() {
                        let n = buffer.parse::<f64>();
                        if n.is_ok() {
                            ret.push(Token::with_value(TokenType::NumericLiteral, &buffer))
                        } else {
                            ret.push(Token::with_value(TokenType::Identifier, &buffer))
                        }
                    }
                    ret.push(Token::new(TokenType::Comma));
                    Some(ret)
                },
                ';' =>
                {
                    let mut ret: Vec<Token> = vec![];
                    if !buffer.is_empty() {
                        let n = buffer.parse::<f64>();
                        if n.is_ok() {
                          ret.push(Token::with_value(TokenType::NumericLiteral, &buffer))
                        } else {
                            ret.push(Token::with_value(TokenType::Identifier, &buffer))
                        }
                    }
                    ret.push(Token::new(TokenType::SemiColon));
                    Some(ret)
                }
                '!' =>  
                {
                    let mut ret: Vec<Token> = vec![];
                    if !buffer.is_empty() {
                        let n = buffer.parse::<f64>();
                        if n.is_ok() {
                          ret.push(Token::with_value(TokenType::NumericLiteral, &buffer))
                        } else {
                            ret.push(Token::with_value(TokenType::Identifier, &buffer))
                        }
                    }
                    if lex_iter.peek().unwrap().1 != '=' {
                        ret.push(Token::new(TokenType::Assign));
                    } else {
                        lex_iter.next();
                        ret.push(Token::new(TokenType::Neq));
                    }
                    Some(ret)
                }
                '=' =>  
                {
                    let mut ret: Vec<Token> = vec![];
                    if !buffer.is_empty() {
                        let n = buffer.parse::<f64>();
                        if n.is_ok() {
                          ret.push(Token::with_value(TokenType::NumericLiteral, &buffer))
                        } else {
                            ret.push(Token::with_value(TokenType::Identifier, &buffer))
                        }
                    }
                    if lex_iter.peek().unwrap().1 != '=' {
                        ret.push(Token::new(TokenType::Assign));
                    } else {
                        lex_iter.next();
                        ret.push(Token::new(TokenType::Eq));
                    }
                    Some(ret)
                }
                '+' =>
                {
                    let mut ret: Vec<Token> = vec![];
                    if !buffer.is_empty() {
                        let n = buffer.parse::<f64>();
                        if n.is_ok() {
                          ret.push(Token::with_value(TokenType::NumericLiteral, &buffer))
                        } else {
                            ret.push(Token::with_value(TokenType::Identifier, &buffer))
                        }
                    }
                    ret.push(Token::new(TokenType::Plus));
                    Some(ret)
                },
                '-' =>
                {
                    let mut ret: Vec<Token> = vec![];
                    if !buffer.is_empty() {
                        let n = buffer.parse::<f64>();
                        if n.is_ok() {
                          ret.push(Token::with_value(TokenType::NumericLiteral, &buffer))
                        } else {
                            ret.push(Token::with_value(TokenType::Identifier, &buffer))
                        }
                    }
                    ret.push(Token::new(TokenType::Minus));
                    Some(ret)
                },
                '*' =>
                {
                    let mut ret: Vec<Token> = vec![];
                    if !buffer.is_empty() {
                        let n = buffer.parse::<f64>();
                        if n.is_ok() {
                          ret.push(Token::with_value(TokenType::NumericLiteral, &buffer))
                        } else {
                            ret.push(Token::with_value(TokenType::Identifier, &buffer))
                        }
                    }
                    ret.push(Token::new(TokenType::Multiply));
                    Some(ret)
                },
                '/' =>
                {
                    let mut ret: Vec<Token> = vec![];
                    if !buffer.is_empty() {
                        let n = buffer.parse::<f64>();
                        if n.is_ok() {
                          ret.push(Token::with_value(TokenType::NumericLiteral, &buffer))
                        } else {
                            ret.push(Token::with_value(TokenType::Identifier, &buffer))
                        }
                    }
                    ret.push(Token::new(TokenType::Divide));
                    Some(ret)
                },
                '"' => 
                {
                    let mut ret: Vec<Token> = vec![];
                    if !buffer.is_empty() {
                        let n = buffer.parse::<f64>();
                        if n.is_ok() {
                          ret.push(Token::with_value(TokenType::NumericLiteral, &buffer))
                        } else {
                            ret.push(Token::with_value(TokenType::Identifier, &buffer))
                        }
                    }
                    Some(ret)

                }
                _ => {
                    buffer.push(*c);
                    None
                }
            };

            if tmp.is_some() {
                for t in tmp.unwrap() {
                    result.push(t);
                }
                buffer.clear();
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use crate::burn_script::tokens::*;
    use super::Lexer;
    #[test]
    fn lex_test_basic() {
        let code = "
        define test_func(x, y) {
            set_variable(i, x);
            loop {
                exit_loop_if(i == y);
                set_variable(i, i + 1);
            }
            exit_script(i);
        }";

        let tokens_expected: Vec<Token> = vec![
            Token::new(TokenType::Define),
            Token::with_value(TokenType::Identifier, "test_func"),
            Token::new(TokenType::OpenParen),
            Token::with_value(TokenType::Identifier, "x"),
            Token::new(TokenType::Comma),
            Token::with_value(TokenType::Identifier, "y"),
            Token::new(TokenType::CloseParen), 
            Token::new(TokenType::OpenBracket),
            Token::with_value(TokenType::Identifier, "set_variable"),
            Token::new(TokenType::OpenParen),
            Token::with_value(TokenType::Identifier, "i"),
            Token::new(TokenType::Comma),
            Token::with_value(TokenType::Identifier, "x"),
            Token::new(TokenType::CloseParen),
            Token::new(TokenType::SemiColon),
            Token::new(TokenType::Loop),
            Token::new(TokenType::OpenBracket),
            Token::with_value(TokenType::Identifier, "exit_loop_if"),
            Token::new(TokenType::OpenParen),
            Token::with_value(TokenType::Identifier, "i"),
            Token::new(TokenType::Eq),
            Token::with_value(TokenType::Identifier, "y"),
            Token::new(TokenType::CloseParen),
            Token::new(TokenType::SemiColon),
            Token::with_value(TokenType::Identifier, "set_variable"),
            Token::new(TokenType::OpenParen),
            Token::with_value(TokenType::Identifier, "i"),
            Token::new(TokenType::Comma),
            Token::with_value(TokenType::Identifier, "i"),
            Token::new(TokenType::Plus),
            Token::with_value(TokenType::NumericLiteral, "1"),
            Token::new(TokenType::CloseParen),
            Token::new(TokenType::SemiColon),
            Token::new(TokenType::CloseBracket),
            Token::with_value(TokenType::Identifier, "exit_script"),
            Token::new(TokenType::OpenParen),
            Token::with_value(TokenType::Identifier, "i"),
            Token::new(TokenType::CloseParen),
            Token::new(TokenType::SemiColon),
            Token::new(TokenType::CloseBracket)
        ];
        let tokens_actual = Lexer::new(code.to_string()).get_tokens();

        /* print non-matching tokens */
        for (i, t) in tokens_expected.iter()
            .enumerate()
                .zip(&tokens_actual)
                .filter(|x| *x.0.1 != *x.1) {
            println!("{}. expected: {:?}    :::     actual: {:?}", i.0, i.1, t);
        }
        assert_eq!(tokens_expected, tokens_actual);
    }
}

