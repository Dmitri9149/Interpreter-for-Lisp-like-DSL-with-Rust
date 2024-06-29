use rustyline::DefaultEditor;
use crate::error::Result;
use crate::configuration::get_configuration;
use log::{debug, info, warn, error};
use pretty_env_logger;
use rustyline::error::ReadlineError;
use std::env::set_var;

pub fn repl() -> Result<()> {

  let configuration = get_configuration().expect("Failed to read configuration.");

    info!("Interpreter Version {}", configuration.readline.version);
    info!("Press CTRL+C or CTRL+D to Exit\n");

    let mut rl = DefaultEditor::new()?;
    if rl.load_history(&configuration.history_file).is_err() {
        info!("No history found");
    }
    
    if configuration.readline.save_history {
      info!("Inputs will be saved to history file");
    } 
    else  {
      info!("Inputs will not be saved to history file");
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
    if configuration.readline.save_history  {rl.save_history(&configuration.history_file)?};

    Ok(())
}

pub fn run() -> Result<()> {

  let configuration = get_configuration().expect("Failed to read configuration.");

  // set the level of Log's 
  set_var("RUST_LOG", configuration.readline.log_level);
  pretty_env_logger::init();

  repl()?;

  Ok(())
}
