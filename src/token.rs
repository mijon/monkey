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
    pub tokentype: TokenType,
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

impl TokenType {
    pub fn literal(&self) -> String {
        match &self {
            TokenType::Illegal => "ILLEGAL".to_string(),
            TokenType::EOF => "~~End of File~~".to_string(),
            TokenType::Identifier(r) => "Identifier ({r})".to_string(),
            TokenType::Int(n) => "Int ({n})".to_string(),
            TokenType::Assign => "Assign".to_string(),
            TokenType::Plus => "+".to_string(),
            TokenType::Minus => "-".to_string(),
            TokenType::FSlash => "/".to_string(),
            TokenType::Asterisk => "*".to_string(),
            TokenType::Comma => ",".to_string(),
            TokenType::Semicolon => ";".to_string(),
            TokenType::Lparen => "(".to_string(),
            TokenType::Rparen => ")".to_string(),
            TokenType::Lbrace => "{".to_string(),
            TokenType::Rbrace => "}".to_string(),
            TokenType::LChevron => "<".to_string(),
            TokenType::RChevron => ">".to_string(),
            TokenType::Exclamation => "!".to_string(),
            TokenType::Function => "function".to_string(),
            TokenType::Let => "let".to_string(),
            TokenType::True => "true".to_string(),
            TokenType::False => "false".to_string(),
            TokenType::If => "if".to_string(),
            TokenType::Else => "else".to_string(),
            TokenType::Return => "return".to_string(),
            TokenType::NewLine => "<newline>".to_string(),
            TokenType::Space => " ".to_string(),
            TokenType::Eq => "==".to_string(),
            TokenType::NotEq => "!=".to_string(),
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.literal())
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
