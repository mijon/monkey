pub mod lexer;
pub mod repl;

use crate::repl::repl::repl;
use lexer::token::{Token, TokenType};

fn main() {
    // let testtoken = TokenType::Identifier(String::from("hello"));
    // let textlexer = lexer::lexer::Lexer::new("hello there");
    //
    // println!("{:?}", textlexer.collect::<Vec<_>>());
    repl();
}
