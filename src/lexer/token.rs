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

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    Function,
    Let,

    NewLine,
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
            TokenType::Comma => write!(f, ","),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Lparen => write!(f, "("),
            TokenType::Rparen => write!(f, ")"),
            TokenType::Lbrace => write!(f, "{{"),
            TokenType::Rbrace => write!(f, "}}"),
            TokenType::Function => write!(f, "function"),
            TokenType::Let => write!(f, "let"),
            TokenType::NewLine => write!(f, "\n"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} @({}:{})", &self.tokentype, &self.line, &self.column)
    }
}
