use rustyline::DefaultEditor;

const HISTORY_FILE_PATH: &str = "./.own_lisp_history.txt";

pub fn repl() -> Result<(), Box<dyn std::error::Error>> {
    println!("Interpreter Version {}", "0.0.0.0.1");
    println!("Press Ctrl+c or Ctrl+d to Exit\n");

    let mut rl = DefaultEditor::new()?;
    if rl.load_history(HISTORY_FILE_PATH).is_err() {
        println!("No history found");
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
