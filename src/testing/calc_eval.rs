use std::{iter::Peekable, sync::mpsc::Iter};

use super::{calc_tokens::{self, Token, TokenType}, test::Variable};

#[derive(Debug, PartialEq)]
pub enum Node {
    Unary { value: String, child: Option<Box<Node>> },
    Binary { left: Box<Node>, operation: TokenType, right: Box<Node> }
}


pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(t: Vec<Token>) -> Self {
        Self {
            tokens: t,
            index: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        if self.index == self.tokens.len() {
            return None;
        }
        Some(&self.tokens[self.index+1])
    }

    fn next(&mut self) -> Option<&Token> {
        if self.index == self.tokens.len() {
            return None;
        }
        self.index += 1;
        Some(&self.tokens[self.index - 1])
    }

    fn current(&self) -> &Token {
        &self.tokens[self.index - 1]
    }

    pub fn parse_identifier(&mut self, tok: Token) -> Result<Box<Node>, &str> {
        let n = self.next();
        // println!("n: {} :: {:?}",n.unwrap().value, n.unwrap().ttype);
        if n.is_none() {
            return Ok(Box::new(Node::Unary { value: self.current().value.clone(), child: None }));
        }
        match n.unwrap().ttype {
            TokenType::Plus => {
                Ok(Box::new(Node::Binary { 
                    left: Box::new(Node::Unary { value: tok.value, child: None }),
                    operation: n.unwrap().ttype, 
                    right: self.parse().expect("Unable to parse.")
                }))
            }
            TokenType::Eq => {
                Ok(Box::new(Node::Binary { 
                    left: Box::new(Node::Unary { value: tok.value, child: None }),
                    operation: n.unwrap().ttype, 
                    right: self.parse().expect("Unable to parse.")
                }))
            },
            TokenType::Neq => {
                Ok(Box::new(Node::Binary { 
                    left: Box::new(Node::Unary { value: tok.value, child: None }),
                    operation: n.unwrap().ttype, 
                    right: self.parse().expect("Unable to parse.")
                }))
            }
            _ => {
                Err("Invalid expression")
            }
        }
    }

    pub fn parse_numeric(&mut self, tok: Token) -> Result<Box<Node>, &str> {
        let n = self.next();
        if n.is_none() {
            return Ok(Box::new(Node::Unary { value: self.current().value.clone(), child: None }));
        }

        match n.unwrap().ttype {
            TokenType::Plus => {
                Ok(Box::new(Node::Binary { 
                    left: Box::new(Node::Unary { value: tok.value, child: None }),
                    operation: n.unwrap().ttype, 
                    right: self.parse().expect("Unable to parse.")
                }))
            },
            TokenType::Eq => {
                Ok(Box::new(Node::Binary { 
                    left: Box::new(Node::Unary { value: tok.value, child: None }),
                    operation: n.unwrap().ttype, 
                    right: self.parse().expect("Unable to parse.")
                }))
            },
            TokenType::Neq => {
                Ok(Box::new(Node::Binary { 
                    left: Box::new(Node::Unary { value: tok.value, child: None }),
                    operation: n.unwrap().ttype, 
                    right: self.parse().expect("Unable to parse.")
                }))
            }
            _ => {
                Err("Invalid expression")
            }
        }
    }

    fn parse_string(&mut self, tok: Token) -> Result<Box<Node>, &str> {
        let n = self.next();
        if n.is_none() {
            return Ok(Box::new(Node::Unary { value: tok.value, child: None }));
        }
        match n.unwrap().ttype {
            calc_tokens::TokenType::Ampersand => {
                Ok(Box::new(Node::Binary { left: Box::new(Node::Unary { value: tok.value, child: None } ), 
                    operation: n.unwrap().ttype, 
                    right: self.parse().expect("unable to parse") }))
            }
            _ => { Err("Unable to perform specified binary operation on string.") }
        }
    }

    pub fn parse(&mut self) -> Result<Box<Node>, &str> {
        let n = self.next();
        if n.is_none() {
            return Err("Nothing to parse.");
        }
        let cur = n.unwrap().clone();
        match cur.ttype {
            calc_tokens::TokenType::Identifier => {
                Ok(self.parse_identifier(cur).expect("Unable to parse identifier"))
            },
            calc_tokens::TokenType::NumericLiteral => {
                Ok(self.parse_numeric(cur).expect("Unable to parse numeric literal"))
            },
            calc_tokens::TokenType::String => {
                Ok(self.parse_string(cur).expect("Unable to parse numeric literal"))
            },
            _ => {
                Err("Unable to parse calculation")
            }
            
        }
    }
}


#[cfg(test)]
mod tests {
    use std::{any::Any, ops::Deref};

    use crate::testing::{calc_tokens::{Token, TokenType}};

    use super::{Node, Parser};

    #[test]
    fn basic_addition() {
        let tokens: Vec<Token> = vec![
            Token::with_value(TokenType::NumericLiteral, "6".to_string()),
            Token::new(TokenType::Plus),
            Token::with_value(TokenType::NumericLiteral, "1".to_string()),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Unable to parse tokens");
        match *ast {
            Node::Binary { ref left, ref operation, ref right } => {
                assert_eq!(*left, Box::new(Node::Unary { value: "6".to_string(), child: None }));
                assert_eq!(*operation, TokenType::Plus);
                assert_eq!(*right, Box::new(Node::Unary { value: "1".to_string(), child: None }));
            }
            _ => {}

        }
    }
}
