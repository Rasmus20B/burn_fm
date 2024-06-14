use decompile::decompiler::decompile_fmp12_file;
use serde::Serialize;
use std::{fs::write, path::Path};

mod component;
mod decompile;
mod file;
mod compile;
mod metadata_constants;
mod chunk;
mod encoding_util;

fn main() {
    let input = Path::new("tests/input/blank.fmp12");
    // let input = Path::new("../fm_vc/databases/Quotes.fmp12");
    let file = decompile_fmp12_file(&input);
    let json = serde_json::to_string_pretty(&file).expect("Unable to generate json file");

    write("test2", json).expect("Unable to write to file.");
}
