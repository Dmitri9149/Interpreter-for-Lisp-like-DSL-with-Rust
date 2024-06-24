use own_lisp::run::{repl,run};
use own_lisp::error;
use std::process::exit;
use log::{debug, info, warn, error};
use pretty_env_logger;


fn main() {
    if let Err(e) = run() {
        eprintln!("Error {}", e);
        exit(1);
    }
}
