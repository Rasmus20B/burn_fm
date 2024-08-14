
use crate::{component::FMComponentScript};
use super::{lexer, parser};

pub struct BurnScriptCompiler {}

impl BurnScriptCompiler {
    pub fn compile_burn_script(code: &str) -> Vec<FMComponentScript> {
        let tokens = lexer::Lexer::new(code.to_string()).get_tokens();
        let scripts = parser::Parser::new(tokens).parse().expect("Unable to parse script block.");
        return scripts;

    }
}
