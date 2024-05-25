use repl::start_repl;
use whoami::username;
mod lexer;
mod repl;


fn main() {
    println!("Hello {}, This is the Monkey programming language!", username());
    start_repl();
}
