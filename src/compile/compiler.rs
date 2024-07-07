use crate::compile::parser;
use crate::compile::lexer;
use crate::file::FmpFile;

pub fn compile_burn(code: &str) -> FmpFile {
    let tokens = lexer::tokenize(&code);
    let p = parser::Parser::new(tokens);
    p.parse_program().expect("unable to parse program. ")
}


