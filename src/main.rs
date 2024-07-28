mod ast;
mod lexer;
mod repl;
mod token;

use crate::repl::repl;
use token::{Token, TokenType};

fn main() {
    // let testtoken = TokenType::Identifier(String::from("hello"));
    // let textlexer = lexer::lexer::Lexer::new("hello there");
    //
    // println!("{:?}", textlexer.collect::<Vec<_>>());
    repl();
}
