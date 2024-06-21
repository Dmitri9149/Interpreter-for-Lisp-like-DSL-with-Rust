use std::process::exit;
use own_lisp::run::repl;

fn main() {
    if let Err(e) = repl() {
        eprintln!("Error {}",e);
        exit(1);
    } 
}
