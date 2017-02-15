use std::io::{self, Write};

use super::tokenizer::Tokenizer;

pub struct Repl {}

impl Repl {
    pub fn run() {
        loop {
            Repl::read_line();
        }
    }

    fn read_line() {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", input);
                println!("{:?}", Tokenizer::tokenize(&input));
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
