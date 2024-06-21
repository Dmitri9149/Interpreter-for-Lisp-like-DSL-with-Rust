use rustyline::DefaultEditor;

const HISTORY_FILE_PATH: &str = "./.own_lisp_history.txt";

pub fn repl() -> Result<(), Box<dyn std::error::Error>> {
    println!("Interpreter Version {}", "0.0.0.0.1");
    println!("Press Ctrl+c or Ctrl+d to Exit\n");

    let mut rl = DefaultEditor::new()?;
    if rl.load_history(HISTORY_FILE_PATH).is_err() {
        println!("No history found");
    }

    println!("To save history of the session , print 'y' + Enter");
    let rl_question = rl
      .readline(">> ");
    let yes = "y".to_string();
    let mut flag:bool = false;
    match rl_question {
      Ok(res) => 
        if res == yes {
          println!("Inputs will be saved to history");
          flag = true;
        } 
        else  {println!("Inputes will not be saved to history");},
      _ => {println!("Inputs will not be saved to history");}
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

    Ok(())
}
