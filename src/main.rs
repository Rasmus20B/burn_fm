use std::{fs::{write, File}, io::Read, path::Path};
use clap::Parser;
use decompile::decompiler::decompile_fmp12_file;

mod cli;
mod component;
mod compile;
mod decompile;
mod file;
mod burn_script;
mod metadata_constants;
mod chunk;
mod encoding_util;
mod fm_script_engine;
mod testing;


fn main() {

    let args = cli::CLI::parse();
    if args.compile.is_some() {
        let path = &args.compile.unwrap()[0];
        let input = Path::new(path);
        let mut code = File::open(input).expect("Unable to open file.");
        let mut text = String::new();
        code.read_to_string(&mut text).expect("Unable to parse file to string");
        let file = compile::compiler::compile_burn(&text);
        let json = serde_json::to_string_pretty(&file).expect("Unable to generate json file");
        write("test_compile", json).expect("Unable to write to file.");
        if args.test {
            let mut env = testing::test::TestEnvironment::new(&file);
            env.generate_tables_for_tests();
        }
    } else if args.decompile.is_some() {
        let path = &args.decompile.unwrap()[0];
        let input = Path::new(path);
        let file = decompile_fmp12_file(&input);
        let json = serde_json::to_string_pretty(&file).expect("Unable to generate json file");
        write("test_decompile_relationships", json).expect("Unable to write to file.");
        if args.test {
            let mut env = testing::test::TestEnvironment::new(&file);
            env.generate_tables_for_tests();
        }
    }

}
