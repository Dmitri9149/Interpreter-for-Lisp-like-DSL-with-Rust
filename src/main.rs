use own_lisp::run::repl;
use own_lisp::error;
use std::process::exit;

fn main() {
    if let Err(e) = repl() {
        eprintln!("Error {}", e);
        exit(1);
    }
}
