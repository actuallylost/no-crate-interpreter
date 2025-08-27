use std::fs;

use crate::{error::Error, token::Token};

pub struct Lexer {
    file_contents: String,
}

impl Lexer {
    pub fn new(file_contents: String) -> Self {
        Self { file_contents }
    }

    /// Returns a Lexer instance from a file.
    pub fn from_path(path: &str) -> Self {
        Self::new(fs::read_to_string(path).unwrap())
    }

    pub fn tokenize(&self) -> Result<Vec<Token>, Error> {
        let mut tokens = vec![];

        let mut chars_iter = self.file_contents.chars().peekable();

        while let Some(c) = chars_iter.next() {
            tokens.push(match c {
                '(' => Token::LParen,
                ')' => Token::RParen,
                '{' => Token::LBrace,
                '}' => Token::RBrace,
                '[' => Token::LBracket,
                ']' => Token::RBracket,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Mul,
                '/' => Token::Div,
                '%' => Token::Mod,
                '=' => Token::Equals,
                '0'..='9' => {
                    let mut chars = String::new();
                    chars.push(c);
                    while let Some(ch) = chars_iter.peek() {
                        if !ch.is_numeric() {
                            break;
                        }
                        chars.push(*ch);
                        chars_iter.next();
                    }
                    Token::Lit(chars.parse().unwrap())
                }
                'A'..='z' => {
                    let mut chars = String::new();
                    chars.push(c);
                    while let Some(ch) = chars_iter.peek() {
                        if !ch.is_alphabetic() {
                            break;
                        }
                        chars.push(*ch);
                        chars_iter.next();
                    }
                    Token::Ident(chars)
                }
                '\r' => {
                    if let Some(ch) = chars_iter.peek() {
                        if !(*ch == '\n') {
                            return Err(Error::UnexpectedEndOfLine('\r'));
                        }
                        chars_iter.next();
                    }
                    Token::CRLF
                }
                '\n' => Token::LF,
                ' ' | '\t' | '\0' => continue,
                _ => {
                    return Err(Error::UnexpectedCharacter(c));
                }
            });
        }

        tokens.push(Token::EOF);
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_char_literals() {
        let lexer = Lexer::new(String::from("1 2 5 8 9 1 7"));
        let expected = vec![
            Token::Lit(1),
            Token::Lit(2),
            Token::Lit(5),
            Token::Lit(8),
            Token::Lit(9),
            Token::Lit(1),
            Token::Lit(7),
            Token::EOF,
        ];
        let actual = lexer.tokenize().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn single_char_literals_leading_zero() {
        let lexer = Lexer::new(String::from("01 02 05 08 09 01 07"));
        let expected = vec![
            Token::Lit(1),
            Token::Lit(2),
            Token::Lit(5),
            Token::Lit(8),
            Token::Lit(9),
            Token::Lit(1),
            Token::Lit(7),
            Token::EOF,
        ];
        let actual = lexer.tokenize().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_char_literals() {
        let lexer = Lexer::new(String::from("12 333 81 741 32"));
        let expected = vec![
            Token::Lit(12),
            Token::Lit(333),
            Token::Lit(81),
            Token::Lit(741),
            Token::Lit(32),
            Token::EOF,
        ];
        let actual = lexer.tokenize().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_char_literals_leading_zero() {
        let lexer = Lexer::new(String::from("012 0333 081 0741 032"));
        let expected = vec![
            Token::Lit(12),
            Token::Lit(333),
            Token::Lit(81),
            Token::Lit(741),
            Token::Lit(32),
            Token::EOF,
        ];
        let actual = lexer.tokenize().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn single_char_identifiers() {
        let lexer = Lexer::new(String::from("a B g o L p K m T D"));
        let expected = vec![
            Token::Ident(String::from("a")),
            Token::Ident(String::from("B")),
            Token::Ident(String::from("g")),
            Token::Ident(String::from("o")),
            Token::Ident(String::from("L")),
            Token::Ident(String::from("p")),
            Token::Ident(String::from("K")),
            Token::Ident(String::from("m")),
            Token::Ident(String::from("T")),
            Token::Ident(String::from("D")),
            Token::EOF,
        ];
        let actual = lexer.tokenize().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_char_identifiers() {
        let lexer = Lexer::new(String::from(
            "awd BJUD gasd odus LaU pUds KadU maDp TaUID Ddua",
        ));
        let expected = vec![
            Token::Ident(String::from("awd")),
            Token::Ident(String::from("BJUD")),
            Token::Ident(String::from("gasd")),
            Token::Ident(String::from("odus")),
            Token::Ident(String::from("LaU")),
            Token::Ident(String::from("pUds")),
            Token::Ident(String::from("KadU")),
            Token::Ident(String::from("maDp")),
            Token::Ident(String::from("TaUID")),
            Token::Ident(String::from("Ddua")),
            Token::EOF,
        ];
        let actual = lexer.tokenize().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn symbols() {
        let lexer = Lexer::new(String::from("+ - * / % = ( ) [ ] { }"));
        let expected = vec![
            Token::Plus,
            Token::Minus,
            Token::Mul,
            Token::Div,
            Token::Mod,
            Token::Equals,
            Token::LParen,
            Token::RParen,
            Token::LBracket,
            Token::RBracket,
            Token::LBrace,
            Token::RBrace,
            Token::EOF,
        ];
        let actual = lexer.tokenize().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn end_of_line() {
        let lexer_1 = Lexer::new(String::from("\r\n"));
        let lexer_2 = Lexer::new(String::from("\n"));
        let expected_1 = vec![Token::CRLF, Token::EOF];
        let expected_2 = vec![Token::LF, Token::EOF];
        let actual_1 = lexer_1.tokenize().unwrap();
        let actual_2 = lexer_2.tokenize().unwrap();

        // CRLF
        assert_eq!(expected_1, actual_1);
        // LF
        assert_eq!(expected_2, actual_2);
    }
}
