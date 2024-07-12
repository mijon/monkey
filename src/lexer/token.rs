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
            TokenType::NewLine => write!(f, "\n"),
            TokenType::Space => write!(f, " "),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} @({}:{})", &self.tokentype, &self.line, &self.column)
    }
}
