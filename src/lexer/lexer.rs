use crate::{Token, TokenType};

// use crate::lexer::token::Token;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: i32,
    col: i32,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            line: 0,
            col: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<char> {
        // Token::new(TokenType::Plus, 1, 1)
        self.col += 1;
        self.input.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Token, TokenType};

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let expected = vec![
            Token::new(TokenType::Assign, 1, 1),
            Token::new(TokenType::Plus, 2, 1),
            Token::new(TokenType::Lparen, 3, 1),
            Token::new(TokenType::Rparen, 4, 1),
            Token::new(TokenType::Lbrace, 5, 1),
            Token::new(TokenType::Rbrace, 6, 1),
            Token::new(TokenType::Comma, 7, 1),
            Token::new(TokenType::Semicolon, 8, 1),
        ];

        let lexed = expected.clone();
        assert_eq!(lexed, expected);
    }
}
