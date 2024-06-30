// use crate::lexer::token::Token;
//
// pub struct Lexer {
//     input: String,
//     position: usize,      // current position in input (points to current char)
//     read_position: usize, // current reading position in input (adter current char)
//     ch: u8,               // current char under examination
// }
//
// impl Lexer {
//     pub fn new(input: String) -> Self {
//         Self {
//             input: input,
//             position: 0,
//             read_position: 1,
//             ch: b'\'',
//         }
//     }
//
//     pub fn read_char(&self) {
//         if self.read_position >= self.input.len() {
//             self.ch = 0
//         } else {
//             self.ch = self.input[self.read_position]
//         };
//         self.position = self.read_position;
//         self.read_position += 1;
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::token::Token;
//
//     #[test]
//     fn test_next_token() {
//         let input = "=+(){},;";
//         let expected = vec![
//             Token::ASSIGN,
//             Token::PLUS,
//             Token::LPAREN,
//             Token::RPAREN,
//             Token::LBRACE,
//             Token::RBRACE,
//             Token::COMMA,
//             Token::SEMICOLON,
//         ];
//     }
// }
