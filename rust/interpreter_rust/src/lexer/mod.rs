#[derive(Debug)]
pub struct Lexer<'a> {
    pub input: &'a String,
    pub position: u128,
    pub read_position: u128,
    pub ch: u8,
}

impl Lexer<'_> {
    pub fn new(&self, input: &String) {
        let mut lexer = Lexer { input: &input.to_string(), position: 0, ch: 0, read_position: 0 };
        lexer.read_char();
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() as u128 {
            self.ch = 0
        } else {
            // let size: usize = 21;
            // self.ch = self.input[self.position]
        }
        self.position = self.read_position;
        self.read_position += 1;
        println!("{:?}", self)
    }
}


