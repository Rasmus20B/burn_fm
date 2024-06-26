use crate::decompile;
use crate::component;
use crate::file;

use crate::compile::token::*;
use crate::compile::parser;
use crate::compile::lexer;

pub fn compile_burn(code: &str) {
    let tokens = lexer::tokenize(&code);
    let tree = parser::parse_program(&tokens);
}


