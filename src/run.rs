use rustyline::DefaultEditor;
use crate::error::Result;
use log::{debug, info, warn, error};
// use pretty_env_logger;
use rustyline::error::ReadlineError;
use env_logger::Env;

const HISTORY_FILE_PATH: &str = "./.own_lisp_history.txt";

pub fn repl() -> Result<()> {

    info!("Interpreter Version {}", "0.0.0.0.1");
    info!("Press CTRL+C or CTRL+D to Exit\n");

    let mut rl = DefaultEditor::new()?;
    if rl.load_history(HISTORY_FILE_PATH).is_err() {
        info!("No history found");
    }
    // ask user to save or not history of the session in history file 
    // for future use 
    info!("To save history of the session to history file, print 'y' + Enter");

    let rl_question = rl
      .readline(">> ");
    let yes = "y".to_string();
    
    let mut save_to_file:bool = false;
    match rl_question {
      Err(ReadlineError::Interrupted) => {
        info!("CTRL-C");
        std::process::exit(0);
      }
      Err(ReadlineError::Eof) => {
        info!("CTRL-D");
        std::process::exit(0);
      },
      Ok(res) => 
        if res == yes {
          info!("Inputs will be saved to history file");
          save_to_file = true;
        } 
        else  {
          info!("Inputs will not be saved to history file");
        }
      _ => info!("Inputs will not be saved to history file")
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
              rl.add_history_entry(&line)?;
              println!("Echo => {}", &line);
            },
            Err(ReadlineError::Interrupted) => {
              info!("CTRL-C");
              break;
            }
            Err(ReadlineError::Eof) => {
                info!("CTRL-D");
                break;
            },
            Err(e) => {
              warn!("Error: {e:?}");
              break;
            },

        }
    }
    if save_to_file  {rl.save_history(HISTORY_FILE_PATH)?};

    Ok(())
}

pub fn run() -> Result<()> {

  let env = Env::default().default_filter_or("info");
  env_logger::Builder::from_env(env).init();

  repl()?;

  Ok(())

}
