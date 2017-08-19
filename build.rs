extern crate menhir;
use std::path::Path;

fn main() {
    let grammar = Path::new("src/parser.rsy");
    menhir::process_file(grammar, &[]);
    menhir::compile_errors(Path::new("errors"), grammar, &[]);
    menhir::cargo_rustc_flags().unwrap();
}
