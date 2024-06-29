use rustyline::DefaultEditor;
use crate::error::Result;
use crate::configuration::get_configuration;
use log::{debug, info, warn, error};
use pretty_env_logger;
use rustyline::error::ReadlineError;
use std::env::set_var;
// use env_logger::Env;

// const HISTORY_FILE_PATH: &str = "./.own_lisp_history.txt";

pub fn repl() -> Result<()> {

  let configuration = get_configuration().expect("Failed to read configuration.");

    info!("Interpreter Version {}", configuration.readline.version);
    info!("Press CTRL+C or CTRL+D to Exit\n");

    let mut rl = DefaultEditor::new()?;
    if rl.load_history(&configuration.history_file).is_err() {
        info!("No history found");
    }
    // ask user to save or not history of the session in history file 
    // for future use 
    info!("To save history of the session to history file, print 'y' + Enter");

    let rl_question = rl
      .readline(&configuration.readline.prompt);
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
        let readline = 
          rl.readline(&configuration.readline.prompt);
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
    if save_to_file  {rl.save_history(&configuration.history_file)?};

    Ok(())
}

pub fn run() -> Result<()> {

  let configuration = get_configuration().expect("Failed to read configuration.");

  // set the level of Log's 
  set_var("RUST_LOG", configuration.readline.log_level);
  pretty_env_logger::init();

//  let env = Env::default().default_filter_or("info");
//  env_logger::Builder::from_env(env).init();

  repl()?;

  Ok(())

}
