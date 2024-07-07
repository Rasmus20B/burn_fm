use super::tokens::Token;


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

    pub fn parse() {

    }

}
