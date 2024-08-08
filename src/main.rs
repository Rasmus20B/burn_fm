#![allow(unused)]
use std::{fs::{write, File}, io::Read, path::Path};
use clap::Parser;
use compile::compiler::compile_burn;
use decompile::decompiler::{decompile_fmp12_file, decompile_fmp12_file_with_header};
use file::FmpFile;

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

    let mut file = FmpFile::new();

    if args.test.is_some() {
        for f in args.test.unwrap() {
            let input = Path::new(&f);
            let mut code = File::open(input).expect("Unable to open file.");
            let mut text = String::new();
            code.read_to_string(&mut text).expect("Unable to parse file to string");
            file.tests.append(&mut compile_burn(&text).tests);
        }
    }

    if args.op.compile.is_some() {
        for f in args.op.compile.unwrap() {
            let input = Path::new(&f);
            let mut code = File::open(input).expect("Unable to open file.");
            let mut text = String::new();
            code.read_to_string(&mut text).expect("Unable to parse file to string");
            let tmp = compile::compiler::compile_burn(&text);
            file.tables.extend(tmp.tables);
            file.relationships.extend(tmp.relationships);
            file.value_lists.extend(tmp.value_lists);
            file.table_occurrences.extend(tmp.table_occurrences);
            file.scripts.extend(tmp.scripts);
            file.layouts.extend(tmp.layouts);
            file.tests.extend(tmp.tests);
            let json = serde_json::to_string_pretty(&file).expect("Unable to generate json file");
            write("test_compile", json).expect("Unable to write to file.");
            if args.no_testing == false && !file.tests.is_empty() {
                let mut env = testing::test::TestEnvironment::new(&file);
                env.generate_test_environment();
                env.run_tests_with_cleanup();
            }
        }
    } else if args.op.decompile.is_some() {
        for f in args.op.decompile.unwrap() {
            let path = f;
            let input = Path::new(&path);
            let tmp: FmpFile;
            if args.print_header == true {
                tmp = decompile_fmp12_file_with_header(&input);
            } else {
                tmp = decompile_fmp12_file(&input);
            }

            let json = serde_json::to_string_pretty(&tmp).expect("Unable to generate json file");
            write("test_decompile", json).expect("Unable to write to file.");
            file.tables.extend(tmp.tables);
            file.relationships.extend(tmp.relationships);
            file.value_lists.extend(tmp.value_lists);
            file.table_occurrences.extend(tmp.table_occurrences);
            file.scripts.extend(tmp.scripts);
            file.layouts.extend(tmp.layouts);
            file.tests.extend(tmp.tests);
            if args.no_testing == false && !file.tests.is_empty() {
                let mut env = testing::test::TestEnvironment::new(&file);
                env.generate_test_environment();
                env.run_tests_with_cleanup();
            }

        }
    }

}
