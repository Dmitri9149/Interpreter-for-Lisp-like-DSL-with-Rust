use rustyline::DefaultEditor;
use crate::error::{Error, Result};
use log::{debug, info, warn, error};
use pretty_env_logger;

const HISTORY_FILE_PATH: &str = "./.own_lisp_history.txt";

pub fn repl() -> Result<()> {

    info!("Interpreter Version {}", "0.0.0.0.1");
    info!("Press Ctrl+c or Ctrl+d to Exit\n");

    let mut rl = DefaultEditor::new()?;
    if rl.load_history(HISTORY_FILE_PATH).is_err() {
        info!("No history found");
    }
    // ask user to save or not history of the session in history file 
    // for future reuse 
    info!("To save history of the session to history file, print 'y' + Enter");

    let rl_question = rl
      .readline(">> ");
    let yes = "y".to_string();
    
    let mut save_to_file:bool = false;
    match rl_question {
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
            }
            Err(e) => {
                println!("Exit, the error => {}", e);
                break;
            }
        }
    }
    if save_to_file  {rl.save_history(HISTORY_FILE_PATH)?};

    Ok(())
}

pub fn run() -> Result<()> {

  pretty_env_logger::init();

  repl()?;

  Ok(())

}
