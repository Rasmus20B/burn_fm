
use std::cell::Cell;

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
    Number(f64),
    Variable(String),
    Field(String),
    StringLiteral(String),
}

pub struct Index {
    val: Cell<usize>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            val: Cell::new(0),
        }
    }

    pub fn get_val(&self) -> usize {
        self.val.get()
    }

    pub fn increment(&self) {
        let cur = self.val.get();
        self.val.set(cur + 1);
    }
}

pub struct TokenList {
    tokens: Vec<Token>,
    index: Index,
}

impl TokenList {
    pub fn new(tokens_: Vec<Token>) -> Self {
        Self {
            tokens: tokens_,
            index: Index::new(),
        }
    }
    fn peek(&self) -> Option<&Token> {
        if self.index.get_val() == self.tokens.len() - 1 {
            return None;
        }
        Some(&self.tokens[self.index.get_val()+1])
    }

    fn next(&self) -> Option<&Token> {
        if self.index.get_val() == self.tokens.len() - 1 {
            return None;
        }
        self.index.increment();
        Some(&self.tokens[self.index.get_val()])
    }

    fn previous(&self) -> Option<&Token> {
        if self.index.get_val() == 0 {
            return None;
        }
        Some(&self.tokens[self.index.get_val() - 1])
    }

    fn current(&self) -> &Token {
        &self.tokens[self.index.get_val()]
    }

}


pub struct Parser {
    tokens: TokenList,
}

impl Parser {
    pub fn new(t: Vec<Token>) -> Self {
        Self {
            tokens: TokenList::new(t),
        }
    }

    fn parse_args(&mut self) -> Vec<Box<Node>> {
        let mut _args = vec![];

        loop {
            self.tokens.next();
            let arg = self.parse_expr().expect("unable to parse argument.");
            println!("arg: {:?}", arg);

            _args.push(arg);

            if !(self.tokens.current().ttype == TokenType::Comma) {
                return _args;
            }
        }
    }

    pub fn parse_func_call(&mut self, _name: String) -> Result<Box<Node>, &str>{
        Ok(Box::new(Node::Call { name: _name, args: self.parse_args() }))
    }

    pub fn parse_identifier(&mut self, tok: Token) -> Result<Box<Node>, &str> {
        let index = &self.tokens.index.get_val();
        let n = self.tokens.next();
        if n.is_none() {
            return Ok(Box::new(Node::Unary { value: self.tokens.current().value.clone(), child: None }));
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

                let operator = self.tokens.next();

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
        let n = self.tokens.next();
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

                let operator = self.tokens.next();

                if operator.is_none() || operator.unwrap().ttype == TokenType::CloseParen {
                    return Ok(func_call)
                }

                let op = operator.unwrap().ttype;
                let expr2 = (self.parse_expr().expect("unable to parse expression."));

                return Ok(Box::new(
                        Node::Binary { 
                            left: func_call,
                            operation: op, 
                            right: expr2 
                }
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
        let n = self.tokens.next();
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

    pub fn parse_comparison(&mut self) -> Result<Box<Node>, &str> {
        let mut lhs = self.parse_expr().expect("Unable to parse expression.");

        println!("lhs of comp: {:?}", lhs);

        while let Some(op) = Some(self.tokens.current().clone()) {
            let n = self.tokens.next();
            if n.is_none() {
                break;
            }
            println!("comparison: {:?}", op);
            match op.ttype {
                TokenType::Eq | TokenType::Neq |
                TokenType::Gt | TokenType::Gtq |
                TokenType::Lt | TokenType::Ltq => {
                    println!("FGets here at some point.");
                    let rhs = self.parse_expr().expect("Unable to parse right hand side of comparison.");
                    lhs = Box::new(Node::Binary { 
                        left: lhs,
                        operation: op.ttype,
                        right: rhs, 
                    })
                }
                _ => {break;}
            }
        }
        Ok(lhs)
    }

    pub fn parse_primary(&mut self) -> Result<Box<Node>, &str> {
        let cur = self.tokens.current();
        /* TODO: Parentheses Parsing */

        println!("looking @ {:?} (next is : {:?})", cur, self.tokens.peek());
        match cur.ttype {
            TokenType::Identifier => {
                if cur.value.split("::").collect::<Vec<_>>().len() == 2 {
                    return Ok(Box::new(Node::Field(cur.value.clone())));
                } else if self.tokens.peek().unwrap_or(&Token::new(TokenType::NumericLiteral)).ttype == TokenType::OpenParen {
                    return Ok(Box::new(Node::Call { name: cur.value.clone(), args: self.parse_args() }))
                } else {
                    return Ok(Box::new(Node::Variable(cur.value.clone())));
                }
            },
            TokenType::NumericLiteral => {
                Ok(Box::new(Node::Number(cur.value.parse().expect("unable to parse number into floating point."))))
            },
            TokenType::String => {
                Ok(Box::new(Node::StringLiteral(cur.value.clone())))
            },
            TokenType::OpenParen => {
                let expr1 = (self.parse_expr().expect("unable to parse grouped expression."));

                let operator = self.tokens.next();

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

    pub fn parse_factor(&mut self) -> Result<Box<Node>, &str> {
        match self.tokens.current().ttype {
            TokenType::NumericLiteral | TokenType::Identifier | TokenType::String => {
                let expr = Ok(self.parse_primary().expect("Unable to parse rhs in factor."));
                self.tokens.next();
                expr
            }
            TokenType::OpenParen => {
                println!("Does get here in fact.");
                self.tokens.next();
                let expr = self.parse_comparison().expect("unable to parse expression.");
                // println!("ex: {:?}", self.tokens.current());
                // if self.tokens.next().unwrap_or(&Token::new(TokenType::Neq)).ttype != TokenType::CloseParen {
                //     return Err("Expected ')'");
                // }
                return Ok(expr);
            }
            _ => { 
                let expr = (self.parse_primary().expect("unable to parse rhs."));
                println!("parsed expr: {:?}", expr);
                // self.tokens.next();
                Ok(expr)
            }
        }
    }

    pub fn parse_term(&mut self) -> Result<Box<Node>, &str> {
        let mut lhs = self.parse_factor().expect("unable to parse factor.");
        while let Some(op) = Some(self.tokens.current()) {
            let op_type = op.ttype;
            println!("op: {:?}", op_type);
            match op_type {
                TokenType::Multiply | TokenType::Divide | TokenType::Ampersand => {
                    self.tokens.next();
                    let rhs = self.parse_factor().expect("unable to parse rhs.");
                    lhs = Box::new(Node::Binary { 
                        left: lhs, 
                        operation: op_type, 
                        right: rhs, 
                    });
                },
                _ => { break; }
            }
        }
        Ok(lhs)
    }

    pub fn parse_expr(&mut self) -> Result<Box<Node>, &str> {
        let mut lhs = self.parse_term().expect("unable to parse lhs term.");
        println!("lhs of term: {:?}, cur: {:?}", lhs, self.tokens.current());

        while let Some(cur) = Some(self.tokens.current()) {
            let op = cur.ttype;
            match cur.ttype {
                TokenType::Plus | TokenType::Minus => {
                    self.tokens.next();
                    let rhs = self.parse_term().expect("unable to parse rhs term.");
                    lhs = Box::new(Node::Binary { 
                        left: lhs, 
                        operation: op, 
                        right: rhs })
                },
                _ => { break; }
            }
        }
        println!("got the expression: {:?}", lhs);

        Ok(lhs)
    }

    pub fn parse(&mut self) -> Result<Box<Node>, &str> {
        let res = self.parse_comparison();
        println!("RESULT: {:?}", res);
        res
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
                assert_eq!(*left, Box::new(Node::Number(6.0)));
                assert_eq!(*operation, TokenType::Plus);
                assert_eq!(*right, Box::new(Node::Number(1.0)));
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
        assert_eq!(Box::new(Node::Binary { 
            left: Box::new(Node::Binary { 
                left: Box::new(Node::Number(3.0)), 
                operation: TokenType::Plus, 
                right: Box::new(Node::Number(2.0)) 
            }), 
            operation: TokenType::Multiply, 
            right: Box::new(Node::Number(4.0)),
            }
        ), ast);
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
                assert_eq!(*left, Box::new(Node::StringLiteral("FileMaker".to_string()))); 
                assert_eq!(*operation, TokenType::Ampersand);
                assert_eq!(*right, Box::new(Node::StringLiteral(" Testing".to_string())));
            }
            _ => { unreachable!() }
        }
    }
}
