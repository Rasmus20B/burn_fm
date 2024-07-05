use decompile::decompiler::decompile_fmp12_file;
use serde::Serialize;
use std::{fs::{write, File}, io::Read, path::Path};

use compile::compiler::compile_burn;

mod component;
mod compile;
mod decompile;
mod file;
mod metadata_constants;
mod chunk;
mod encoding_util;
mod script_engine;
mod testing;

fn main() {
    let input = Path::new("tests/input/blank.fmp12");
    // let input = Path::new("../fm_vc/databases/Quotes.fmp12");
    // let input = Path::new("example.burn");

    // let mut code = File::open(input).expect("Unable to open file.");
    // let mut text = String::new();
    // code.read_to_string(&mut text).expect("Unable to parse file to string");

    // let file = compile_burn(&text);

    let file = decompile_fmp12_file(&input);
    println!("read file");
    let json = serde_json::to_string_pretty(&file).expect("Unable to generate json file");
    println!("got the json");
    write("test2", json).expect("Unable to write to file.");

    let mut env = testing::test::TestEnvironment::new(&file);
    env.generate_tables_for_tests();
}
