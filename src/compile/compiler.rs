use crate::decompile;
use crate::component;
use crate::file;

use crate::compile::token::*;
use crate::compile::lexer;

pub fn compile_burn(code: &str) {
    let tokens = lexer::tokenize(code);
}


