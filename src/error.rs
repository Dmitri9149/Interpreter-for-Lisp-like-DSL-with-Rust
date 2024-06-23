use std::fmt::{self, Debug};

#[derive(Debug)]
pub enum Error {
  Readline(String),
  IO(String),
}

pub type Result<T> = std::result::Result<T,Error>;

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::Readline(s) => write!(f, "Readline error {}", s),
      Error::IO(s) => write!(f, "IO error {}", s),
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(error: std::io::Error) -> Self {
    Error::IO(error.to_string())
  }
}

impl From<rustyline::error::ReadlineError> for Error {
  fn from(error: rustyline::error::ReadlineError) -> Self {
    Error::Readline(error.to_string())
  }
}