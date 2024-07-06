use std::{fs::{read, write, File}, io::Read, path::Path};
use decompile::decompiler::decompile_fmp12_file;
use clap::{arg, Parser, command, Command};

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

#[derive(Parser)]
struct CLI {
    #[clap(short = 'c', conflicts_with="decompile")]
    compile: Option<Vec<String>>,
    #[clap(short = 'd', conflicts_with="compile")]
    decompile: Option<Vec<String>>,
    #[clap(short)]
    test: bool,
}

fn main() {

    let args = CLI::parse();
    if args.compile.is_some() {
        let path = &args.compile.unwrap()[0];
        let input = Path::new(path);
        let mut code = File::open(input).expect("Unable to open file.");
        let mut text = String::new();
        code.read_to_string(&mut text).expect("Unable to parse file to string");
        
        let file = compile::compiler::compile_burn(&text);
    }  else if args.decompile.is_some() {
        let path = &args.decompile.unwrap()[0];
        let input = Path::new(path);
        // let input = Path::new("../fm_vc/databases/Quotes.fmp12");

        let file = decompile_fmp12_file(&input);
        println!("read file");
        let json = serde_json::to_string_pretty(&file).expect("Unable to generate json file");
        println!("got the json");
        write("test2", json).expect("Unable to write to file.");
    }

    // if args.test == true {
    //     let mut env = testing::test::TestEnvironment::new(&file);
    //     env.generate_tables_for_tests();
    // }

}
