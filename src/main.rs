use decompile::decompiler::decompile_fmp12_file;
use std::path::Path;

mod component;
mod decompile;
mod compile;
mod file;

fn main() {
    let file = Path::new("tests/input/blank.fmp12");
    decompile_fmp12_file(&file);
}
