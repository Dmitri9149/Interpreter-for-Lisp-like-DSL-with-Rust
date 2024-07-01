use own_lisp::run::run;
use std::process::exit;

use own_lisp::parser::parse;



fn main() {

    parse::simple_examples();

    if let Err(e) = run() {
        eprintln!("Error {}", e);
        exit(1);
    }
}
