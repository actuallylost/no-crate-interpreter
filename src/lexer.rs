use std::{fs, str::Chars};

use crate::{error::Error, token::Token};

#[derive(PartialEq, Clone, Copy)]
enum CharType {
    Alpha,
    Num,
}

pub struct Lexer {
    current: usize,
    file_contents: String,
}

impl Lexer {
    pub fn new(current: usize, file_contents: String) -> Self {
        Self {
            current,
            file_contents,
        }
    }

    /// Returns a Lexer instance from a file.
    pub fn from_file(path: &str) -> Self {
        Self::new(0, fs::read_to_string(path).unwrap())
    }

    fn check_next_char(&self, init: char, iter: &mut Chars, ty: CharType) -> String {
        // if self.current == self.file_contents.len() {
        //     return Err(());
        // }

        let mut chars = String::from(init);

        for c in iter {
            if ty == CharType::Num {
                match c {
                    '0'..='9' => chars.push_str(&c.to_string()),
                    _ => return chars,
                };
            } else {
                match c {
                    'A'..='z' => chars.push_str(&c.to_string()),
                    _ => return chars,
                };
            }
            // self.current += 1
        }

        chars
    }

    pub fn tokenize(&self) -> Result<Vec<Token>, Error> {
        let mut tokens = vec![];
        let mut chars_iter = self.file_contents.chars();

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
                '=' => Token::Assign,
                '0'..='9' => Token::Lit(self.check_next_char(c, &mut chars_iter, CharType::Num)),
                'A'..='z' => Token::Ident(self.check_next_char(c, &mut chars_iter, CharType::Num)),
                ' ' | '\n' | '\t' => continue,
                _ => {
                    return Err(Error::UnknownCharacterError(c));
                }
            });
        }

        Ok(tokens)
    }
}
