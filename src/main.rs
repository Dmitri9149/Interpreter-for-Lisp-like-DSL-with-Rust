use own_lisp::run::repl;
use std::process::exit;

fn main() {
    if let Err(e) = repl() {
        eprintln!("Error {}", e);
        exit(1);
    }
}
