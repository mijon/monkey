use crate::{Token, TokenType};

// use crate::lexer::token::Token;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            line: 0,
            col: 0,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.col += 1;

        if let Some(output_char) = self.input.next() {
            Some(Token::new(
                TokenType::Identifier(output_char.to_string()),
                self.line,
                self.col,
            ))
        } else {
            None
        }
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
