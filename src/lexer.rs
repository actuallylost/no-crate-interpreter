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
                '[' => Token::LBrace,
                ']' => Token::RBrace,
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
                ' ' | '\n' | '\t' | '\0' => continue,
                _ => {
                    return Err(Error::UnknownCharacterError(c));
                }
            });
        }

        Ok(tokens)
    }
}
