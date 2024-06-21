use rustyline::DefaultEditor;

const HISTORY_FILE_PATH: &str = "./.own_lisp_history.txt";

pub fn repl() -> Result<(), Box<dyn std::error::Error>>  {
  let mut rl = DefaultEditor::new()?;

  if rl.load_history(HISTORY_FILE_PATH).is_err() {
    println!("No command history found");
  }

  loop {
    let readline = rl.readline(">> ");
    match readline {
      Ok(line) => println!("Echo => {}", &line),
      Err(e) => { 
        println!("Exit, the error => {}", e); 
        break;
      }
    }
  }

  Ok(())

}