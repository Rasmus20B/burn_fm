
#[derive(Debug, PartialEq)]
pub enum TokenType {
    Define,
    Identifier,
    OpenParen,
    CloseParen,
    SemiColon,
    Comma,
    OpenBracket,
    CloseBracket,
    Let,
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,
    Loop,
    Eq,
    Neq,
    Geq,
    Leq,
    Gt,
    Lt,
    NumericLiteral,
}

#[derive(PartialEq, Debug)]
pub struct Token {
    ttype: TokenType,
    value: String,
}

impl Token {
    pub fn new(t: TokenType) -> Self {
        Self {
            ttype: t,
            value: "".to_string(),
        }
    }
    pub fn with_value(t:TokenType, val: &str) -> Self {
        Self {
            ttype: t,
            value: val.to_string(),
        }
    }
}
