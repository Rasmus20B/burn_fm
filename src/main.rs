use decompile::decompiler::decompile_fmp12_file;
use std::path::Path;

mod component;
mod decompile;
mod compile;
mod file;
mod metadata_constants;

fn main() {
    let input = Path::new("tests/input/blank.fmp12");
    // let input = Path::new("../fm_vc/databases/Quotes.fmp12");
    let file = decompile_fmp12_file(&input);

    for t in file.tables {
        println!("Table name: {}", t.1.table_name);
        for f in t.1.fields {
            println!("\tfield: {}: {}", f.1.field_name, f.1.field_description);
        }
    }

}
