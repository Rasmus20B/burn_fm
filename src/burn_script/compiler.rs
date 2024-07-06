
use crate::fm_script_engine::fm_script_engine_instructions::Instruction;

use super::lexer;

pub struct BurnScriptCompiler {
}

impl BurnScriptCompiler {

    pub fn compile_burn_script(code: &str) {
        let tokens = lexer::Lexer::new(code.to_string()).get_tokens();
        
    }

    pub fn compile_fm_script(code: Vec<Instruction>) {

    }
}
