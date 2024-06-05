use decompile::decompiler::decompile_fmp12_file;
use std::path::Path;

mod component;
mod decompile;
mod compile;
mod file;

fn main() {
    let file = Path::new("../fm_vc/databases/Quotes.fmp12");
    decompile_fmp12_file(&file);
}
