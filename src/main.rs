extern crate tricktracktruck_planning_import as importer;

use std::env;

fn main() {
    let filepath = match env::args().nth(0) {
        None => {
            eprintln!("Requires at least one argument");
            return;
        },
        Some(arg) => arg
    };
    match importer::parser::read_file(&filepath) {
        Err(e)        => eprintln!("Error: {:?}", e),
        Ok(rotations) => {
            println!("Success !");
            for rotation in rotations {
                println!("  {:?}", rotation);
            }
        }
    }
}
