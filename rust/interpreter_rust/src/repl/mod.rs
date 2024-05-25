use std::io::{stdin};
pub  fn start_repl () {
    println!("repl start");
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("please provider me to the input");
        println!("You typed: {}", input);
    }
}
