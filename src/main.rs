use decompile::decompiler::decompile_fmp12_file;
use std::path::Path;

mod component;
mod decompile;
mod compile;
mod file;
mod metadata_constants;

extern crate num;
#[macro_use]
extern crate num_derive;

fn main() {
    let input = Path::new("tests/input/blank.fmp12");
    let file = decompile_fmp12_file(&input);

    for t in file.tables {
        println!("Table name: {}", t.table_name);
        for f in t.fields {
            println!("\tfield: {}: {}", f.1.field_name, f.1.field_description);
        }
    }

}
