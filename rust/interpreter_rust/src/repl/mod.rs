use std::io::stdin;

use crate::lexer;

pub fn start_repl() {
    println!("repl start");
    loop {
        print!(">>");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("please provider me to the input");
        let mut lexer = lexer::Lexer { input: &input, read_position: 0, position: 0, ch: 0 };
        lexer.read_char();
        println!("You typed: {}", input);
    }
}
