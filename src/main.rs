pub mod lexer;

use lexer::token::{Token, TokenType};

fn main() {
    // let testtoken = TokenType::Identifier(String::from("hello"));
    let textlexer = lexer::lexer::Lexer::new("+=hello");

    println!("{:?}", textlexer.collect::<Vec<_>>());
}
