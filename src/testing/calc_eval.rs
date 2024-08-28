
use super::calc_tokens::{self, Token, TokenType};

#[derive(Debug, PartialEq)]
pub enum Node {
    Unary { value: String, child: Option<Box<Node>> },
    Binary { left: Box<Node>, operation: TokenType, right: Box<Node> },
    Grouping { left: Box<Node>, operation: TokenType, right: Box<Node> }
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

    fn previous(&self) -> Option<&Token> {
        if self.index == 0 {
            return None;
        }
        Some(&self.tokens[self.index - 1])
    }

    fn current(&self) -> &Token {
        &self.tokens[self.index - 1]
    }

    pub fn parse_identifier(&mut self, tok: Token) -> Result<Box<Node>, &str> {
        let n = self.next();
        if n.is_none() {
            return Ok(Box::new(Node::Unary { value: self.current().value.clone(), child: None }));
        }
        match n.unwrap().ttype {
            TokenType::Eq | TokenType::Neq | TokenType::Gt 
                | TokenType::Gtq | TokenType::Lt | TokenType::Ltq 
                | TokenType::Plus | TokenType::Minus | TokenType::Multiply
                | TokenType::Divide | TokenType::Ampersand => {
                Ok(Box::new(Node::Binary { 
                    left: Box::new(Node::Unary { value: tok.value, child: None }),
                    operation: n.unwrap().ttype, 
                    right: self.parse().expect("Unable to parse.")
                }))
            },
            _ => {
                Err("Invalid expression")
            }
        }
    }

    pub fn parse_numeric(&mut self, tok: Token) -> Result<Box<Node>, &str> {
        let n = self.next();
        if n.is_none() {
            return Ok(Box::new(Node::Unary { value: tok.value.clone(), child: None }));
        }

        match n.unwrap().ttype {
            TokenType::Eq | TokenType::Neq | TokenType::Gt 
                | TokenType::Gtq | TokenType::Lt | TokenType::Ltq 
                | TokenType::Plus | TokenType::Minus | TokenType::Multiply
                | TokenType::Divide | TokenType::Ampersand => {
                Ok(Box::new(Node::Binary { 
                    left: Box::new(Node::Unary { value: tok.value, child: None }),
                    operation: n.unwrap().ttype, 
                    right: self.parse_expression().expect("Unable to parse.")
                }))
            },
            TokenType::CloseParen => {
                Ok(Box::new(Node::Unary { 
                    value: tok.value.clone(), 
                    child: None 
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
            return Ok(Box::new(Node::Unary { value: format!("{}", tok.value), child: None }));
        }
        match n.unwrap().ttype {
            TokenType::Ampersand => {
                Ok(Box::new(Node::Binary { 
                    left: Box::new(Node::Unary { value: format!("{}", tok.value), child: None } ), 
                    operation: n.unwrap().ttype, 
                    right: self.parse().expect("unable to parse") 
                }))
            },
            TokenType::Eq => {
                Ok(Box::new(Node::Binary { 
                    left: Box::new(Node::Unary { value: format!("{}", tok.value), child: None } ), 
                    operation: n.unwrap().ttype,
                    right: self.parse().expect("Uable to parse")
                }))
            }
            _ => { Err("Unable to perform specified binary operation on string.") }
        }
    }

    pub fn parse_grouping(&mut self) -> Result<Box<Node>, &str> {

        let n = self.peek();
        if n.is_none() {
            return Err("unterminated parenthesis.");
        }
        Ok(self.parse_expression().expect("Unable to parse subparen."))
    }

    pub fn parse_expression(&mut self) -> Result<Box<Node>, &str> {
        let n = self.next();
        if n.is_none() {
            return Err("Nothing to parse.");
        }
        let cur = n.unwrap().clone();
        /* TODO: Parentheses Parsing */
        match cur.ttype {
            TokenType::Identifier => {
                Ok(self.parse_identifier(cur).expect("Unable to parse identifier"))
            },
            TokenType::NumericLiteral => {
                Ok(self.parse_numeric(cur).expect("Unable to parse numeric literal"))
            },
            TokenType::String => {
                Ok(self.parse_string(cur).expect("Unable to parse String literal"))
            },
            TokenType::OpenParen => {
                let expr1 = (self.parse_expression().expect("unable to parse grouped expression."));
                let operator = self.next();
                let op = operator.unwrap().ttype;
                let expr2 = (self.parse_expression().expect("unable to parse grouped expression."));

                Ok(Box::new( Node::Grouping { 
                    left: expr1, 
                    operation: op, 
                    right: expr2 
                }))
            }
            _ => {
                Err("Unable to parse calculation: ")
            }
        }
    }

    pub fn parse(&mut self) -> Result<Box<Node>, &str> {
        self.parse_expression()
    }
}


#[cfg(test)]
mod tests {
    use crate::testing::calc_tokens::{Token, TokenType};
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

    #[test]
    fn group_arithmetic() {
        let tokens: Vec<Token> = vec![
            Token::new(TokenType::OpenParen),
            Token::with_value(TokenType::NumericLiteral, 3.to_string()),
            Token::new(TokenType::Plus),
            Token::with_value(TokenType::NumericLiteral, 2.to_string()),
            Token::new(TokenType::CloseParen),
            Token::new(TokenType::Multiply),
            Token::with_value(TokenType::NumericLiteral, 4.to_string()),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Unable to parse tokens");
        assert_eq!(Box::new(Node::Grouping { 
            left: Box::new(Node::Binary { 
                left: Box::new(Node::Unary { value: 3.to_string(), child: None }), 
                operation: TokenType::Plus, 
                right: Box::new(Node::Unary { value: 2.to_string(), child: None }) }), 
            operation: TokenType::Multiply, 
            right: Box::new(Node::Unary { value: 4.to_string(), child: None }) 
        }), ast);
    }

    #[test]
    fn string_concat() {
        let tokens: Vec<Token> = vec![
            Token::with_value(TokenType::String, "FileMaker".to_string()),
            Token::new(TokenType::Ampersand),
            Token::with_value(TokenType::String, " Testing".to_string()),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Unable to parse tokens");
        match *ast {
            Node::Binary { ref left, ref operation, ref right } => {
                assert_eq!(*left, Box::new(Node::Unary { value: "FileMaker".to_string(), child: None }));
                assert_eq!(*operation, TokenType::Ampersand);
                assert_eq!(*right, Box::new(Node::Unary { value: " Testing".to_string(), child: None }));
            }
            _ => { unreachable!() }
        }
    }
}
