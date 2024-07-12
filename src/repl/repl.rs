use crate::lexer::lexer;
use std::io;
use std::io::Write;

const PROMPT: &'static str = ">> ";

pub fn repl() {
    loop {
        print!("{}", PROMPT);
        std::io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input == "/exit\n" {
            break;
        }

        let lexed = lexer::Lexer::new(&input);
        for token in lexed.collect::<Vec<_>>() {
            println!("{}", token)
        }
    }
}
