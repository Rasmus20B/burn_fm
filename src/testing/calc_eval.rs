
use super::calc_tokens::{self, Token, TokenType};

enum Precedence {
    Lowest,
    Add,
    Subtract,
    Multiply,
    Divide,
    Paren,
    Concatenate,
}

impl Precedence {
    pub fn from_int(num: usize) -> Result<Self, String> {
        match num {
            0 => Ok(Precedence::Lowest),
            1 => Ok(Precedence::Add),
            2 => Ok(Precedence::Subtract),
            3 => Ok(Precedence::Add),
            4 => Ok(Precedence::Subtract),
            5 => Ok(Precedence::Paren),
            6 => Ok(Precedence::Concatenate),
            _ => Err("invalid integer".to_string())
        }
    }
    pub fn to_int(num: Self) -> usize {
        match num {
            Precedence::Lowest => 0,
            Precedence::Add => 1,
            Precedence::Subtract => 2,
            Precedence::Add => 3,
            Precedence::Subtract => 4,
            Precedence::Paren => 5,
            Precedence::Concatenate => 6,
            _ => unreachable!()
        }
    }

}

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Unary { value: String, child: Option<Box<Node>> },
    Binary { left: Box<Node>, operation: TokenType, right: Box<Node> },
    Grouping { left: Box<Node>, operation: TokenType, right: Box<Node> },
    Call { name: String, args: Vec<Box::<Node>> },
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

    fn parse_args(&mut self) -> Vec<Box<Node>> {
        let mut _args = vec![];

        loop {
            _args.push(self.parse_expr().expect("Unable to parse argument"));

            if !(self.tokens[self.index - 1].ttype == TokenType::Comma) {
                return _args;
            }
        }
    }

    pub fn parse_func_call(&mut self, _name: String) -> Result<Box<Node>, &str>{
        Ok(Box::new(Node::Call { name: _name, args: self.parse_args() }))
    }

    pub fn parse_identifier(&mut self, tok: Token) -> Result<Box<Node>, &str> {
        let index = self.index;
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
            TokenType::OpenParen => {

                let func_call = self.parse_func_call(tok.value).expect("Unable to parse function call");

                let operator = self.next();

                if operator.is_none() || operator.unwrap().ttype == TokenType::CloseParen {
                    return Ok(func_call)
                }

                let op = operator.unwrap().ttype;
                let expr2 = (self.parse_expr().expect("unable to parse expression."));

                return Ok(Box::new(
                        Node::Binary { 
                            left: func_call,
                            operation: op, 
                            right: expr2 }
                ));
                // return Ok(self.parse_func_call(tok.value).expect("Unable to parse function call"));
            }
            TokenType::CloseParen => {
                Ok(Box::new(Node::Unary { 
                    value: tok.value.clone(), 
                    child: None 
                }))
            },
            TokenType::Comma => {
                Ok(Box::new(Node::Unary { 
                    value: tok.value.clone(), 
                    child: None 
                }))
            },
            _ => {
                println!("ident: {:?}", n);
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
                    right: self.parse_expr().expect("Unable to parse.")
                }))
            },
            TokenType::OpenParen => {

                let func_call = self.parse_func_call(tok.value).expect("Unable to parse function call");

                let operator = self.next();

                if operator.is_none() || operator.unwrap().ttype == TokenType::CloseParen {
                    return Ok(func_call)
                }

                let op = operator.unwrap().ttype;
                let expr2 = (self.parse_expr().expect("unable to parse expression."));

                return Ok(Box::new(
                        Node::Binary { 
                            left: func_call,
                            operation: op, 
                            right: expr2 }
                ));
                // return Ok(self.parse_func_call(tok.value).expect("Unable to parse function call"));
            }
            TokenType::CloseParen => {
                Ok(Box::new(Node::Unary { 
                    value: tok.value.clone(), 
                    child: None 
                }))
            },
            TokenType::Comma => {
                Ok(Box::new(Node::Unary { 
                    value: tok.value.clone(), 
                    child: None 
                }))
            },
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
            _ => { 
                Err("Unable to perform specified binary operation on string.") 
            }
        }
    }

    pub fn parse_primary(&mut self) -> Result<Box<Node>, &str> {
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
                let expr1 = (self.parse_expr().expect("unable to parse grouped expression."));

                let operator = self.next();

                if operator.is_none() || operator.unwrap().ttype == TokenType::CloseParen || operator.unwrap().ttype == TokenType::Comma {
                    return Ok(expr1)
                }

                let op = operator.unwrap().ttype;
                let expr2 = (self.parse_expr().expect("unable to parse grouped expression."));

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

    // pub fn parse_binary_expr(&mut self, prec: Precedence) -> Result<Box<Node>, &str> {
    //
    // }

    pub fn parse_expr(&mut self) -> Result<Box<Node>, &str> {
        Ok(self.parse_primary().expect("Unable to parse primary expression."))
    }

    pub fn parse(&mut self) -> Result<Box<Node>, &str> {
        self.parse_expr()
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
