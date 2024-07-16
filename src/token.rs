use core::fmt;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Illegal,
    EOF,

    Identifier(String),
    Int(usize),

    Assign,
    Plus,
    Minus,
    Asterisk,
    FSlash,

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    LChevron,
    RChevron,

    Exclamation,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,

    NewLine,
    Space,

    Eq,
    NotEq,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    tokentype: TokenType,
    line: usize,
    column: usize,
}

impl Token {
    pub fn new(tokentype: TokenType, line: usize, column: usize) -> Self {
        Self {
            tokentype,
            line,
            column,
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TokenType::Illegal => write!(f, "ILLEGAL"),
            TokenType::EOF => write!(f, "~~End of File~~"),
            TokenType::Identifier(r) => write!(f, "Identifier ({r})"),
            TokenType::Int(n) => write!(f, "Int ({})", n),
            TokenType::Assign => write!(f, "Assign"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::FSlash => write!(f, "/"),
            TokenType::Asterisk => write!(f, "*"),
            TokenType::Comma => write!(f, ","),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Lparen => write!(f, "("),
            TokenType::Rparen => write!(f, ")"),
            TokenType::Lbrace => write!(f, "{{"),
            TokenType::Rbrace => write!(f, "}}"),
            TokenType::LChevron => write!(f, "<"),
            TokenType::RChevron => write!(f, ">"),
            TokenType::Exclamation => write!(f, "!"),
            TokenType::Function => write!(f, "function"),
            TokenType::Let => write!(f, "let"),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            TokenType::If => write!(f, "if"),
            TokenType::Else => write!(f, "else"),
            TokenType::Return => write!(f, "return"),
            TokenType::NewLine => write!(f, "<newline>"),
            TokenType::Space => write!(f, " "),
            TokenType::Eq => write!(f, "=="),
            TokenType::NotEq => write!(f, "!="),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} @({}:{})", &self.tokentype, &self.line, &self.column)
    }
}

pub fn tokens_to_tokentypes(tokens: &Vec<Token>) -> Vec<TokenType> {
    tokens
        .into_iter()
        .map(|v| v.tokentype.clone())
        .collect::<Vec<TokenType>>()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    // use crate::{lexer, Token, TokenType};
    use super::*;
    #[test]
    fn test_token_type_extraction() {
        let token_vector = vec![
            Token::new(TokenType::Function, 1, 1),
            Token::new(TokenType::Identifier("test".to_string()), 1, 5),
            Token::new(TokenType::Lparen, 1, 7),
        ];

        let expected = vec![
            TokenType::Function,
            TokenType::Identifier("test".to_string()),
            TokenType::Lparen,
        ];

        assert_eq!(tokens_to_tokentypes(&token_vector), expected)
    }
}
