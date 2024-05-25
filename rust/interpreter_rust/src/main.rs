use whoami::username;

mod lexer;
mod repl;


fn main() {
    println!("Hello {}, This is the Monkey programming language!", username());
    // start_repl();

    let mut my_string = String::new();
    my_string = "Hellow".parse().unwrap();
}

fn print_screen(data: &String) {
    println!("{}", data)
}
