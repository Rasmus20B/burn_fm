use decompile::decompiler::decompile_fmp12_file;
use serde::Serialize;
use std::{fs::write, path::Path};

mod component;
mod decompile;
mod compile;
mod file;
mod metadata_constants;

fn main() {
    // let input = Path::new("tests/input/blank.fmp12");
    let input = Path::new("../fm_vc/databases/Quotes.fmp12");
    let file = decompile_fmp12_file(&input);
    let json = serde_json::to_string_pretty(&file).expect("Unable to generate json file");

    write("test2", json).expect("Unable to write to file.");
}
