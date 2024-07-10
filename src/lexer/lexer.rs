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
            line: 1,
            col: 0,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.col += 1;

        if let Some(output_char) = self.input.next() {
            if output_char.is_alphabetic() {
                let start_col = self.col;

                let mut temp_string = String::new();
                temp_string.push(output_char);

                // TODO: there's got to be a nicer way of doing this, come back and fix it.
                while let Some(peeked) = self.input.peek() {
                    if peeked.is_alphabetic() {
                        temp_string.push(self.input.next().unwrap());
                        self.col += 1;
                    } else {
                        break;
                    }
                }

                let token_type = match temp_string.as_str() {
                    "fn" => TokenType::Function,
                    "let" => TokenType::Let,
                    _ => TokenType::Identifier(temp_string),
                };

                Some(Token::new(token_type, self.line, start_col))
            } else if output_char.is_numeric() {
                let start_col = self.col;

                let mut temp_num = String::new();
                temp_num.push(output_char);

                while let Some(peeked) = self.input.peek() {
                    if peeked.is_numeric() {
                        temp_num.push(self.input.next().unwrap());
                        self.col += 1;
                    } else {
                        break;
                    }
                }

                Some(Token::new(
                    TokenType::Int(temp_num.parse().unwrap()),
                    self.line,
                    start_col,
                ))
            } else {
                let token_type = match output_char {
                    '=' => TokenType::Assign,
                    '+' => TokenType::Plus,
                    ',' => TokenType::Comma,
                    ';' => TokenType::Semicolon,
                    '(' => TokenType::Lparen,
                    ')' => TokenType::Rparen,
                    '{' => TokenType::Lbrace,
                    '}' => TokenType::Rbrace,
                    '\n' => {
                        self.line += 1;
                        self.col = 0;
                        TokenType::NewLine
                    }
                    ' ' => TokenType::Space,
                    _ => {
                        println!("char: {}.", output_char);
                        todo!();
                    }
                };
                Some(Token::new(token_type, self.line, self.col))
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::lexer, Token, TokenType};
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let expected = vec![
            Token::new(TokenType::Assign, 1, 1),
            Token::new(TokenType::Plus, 1, 2),
            Token::new(TokenType::Lparen, 1, 3),
            Token::new(TokenType::Rparen, 1, 4),
            Token::new(TokenType::Lbrace, 1, 5),
            Token::new(TokenType::Rbrace, 1, 6),
            Token::new(TokenType::Comma, 1, 7),
            Token::new(TokenType::Semicolon, 1, 8),
        ];

        let lexed = lexer::Lexer::new(input);
        assert_eq!(lexed.collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_one_line_code() {
        let input = "let five = 5;";
        let expected = vec![
            Token::new(TokenType::Let, 1, 1),
            Token::new(TokenType::Space, 1, 4),
            Token::new(TokenType::Identifier("five".to_string()), 1, 5),
            Token::new(TokenType::Space, 1, 9),
            Token::new(TokenType::Assign, 1, 10),
            Token::new(TokenType::Space, 1, 11),
            Token::new(TokenType::Int(5), 1, 12),
            Token::new(TokenType::Semicolon, 1, 13),
        ];

        let lexed = lexer::Lexer::new(input);
        assert_eq!(lexed.collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_two_line_code() {
        let input = "let five = 5;
let ten = 10;";
        let expected = vec![
            Token::new(TokenType::Let, 1, 1),
            Token::new(TokenType::Space, 1, 4),
            Token::new(TokenType::Identifier("five".to_string()), 1, 5),
            Token::new(TokenType::Space, 1, 9),
            Token::new(TokenType::Assign, 1, 10),
            Token::new(TokenType::Space, 1, 11),
            Token::new(TokenType::Int(5), 1, 12),
            Token::new(TokenType::Semicolon, 1, 13),
            Token::new(TokenType::NewLine, 1, 14),
            Token::new(TokenType::Let, 2, 1),
            Token::new(TokenType::Space, 2, 4),
            Token::new(TokenType::Identifier("ten".to_string()), 2, 5),
            Token::new(TokenType::Space, 2, 8),
            Token::new(TokenType::Assign, 2, 9),
            Token::new(TokenType::Space, 2, 10),
            Token::new(TokenType::Int(10), 2, 11),
            Token::new(TokenType::Semicolon, 2, 13),
        ];

        let lexed = lexer::Lexer::new(input);
        assert_eq!(lexed.collect::<Vec<_>>(), expected);
    }
}
