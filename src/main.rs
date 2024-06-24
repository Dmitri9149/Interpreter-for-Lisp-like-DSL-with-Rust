use own_lisp::run::{repl,run};
use own_lisp::error;
use std::process::exit;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error {}", e);
        exit(1);
    }
}
