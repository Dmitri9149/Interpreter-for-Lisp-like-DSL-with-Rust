use own_lisp::run::run;
use std::process::exit;



fn main() {
    if let Err(e) = run() {
        eprintln!("Error {}", e);
        exit(1);
    }
}
