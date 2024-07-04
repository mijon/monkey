pub mod lexer;

use lexer::token::{Token, TokenType};

fn main() {
    // let testtoken = TokenType::Identifier(String::from("hello"));
    let mut textlexer = lexer::lexer::Lexer::new("+=hello");

    let testtoken = Token::new(TokenType::Plus, 1, 1);

    println!("{:?}", testtoken);
    println!("{}", testtoken);
    println!("{:?}", textlexer);
}
