pub fn repl() -> Result<(), Box<dyn std::error::Error>>  {
  let mut rl = rustyline::DefaultEditor::new()?;

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