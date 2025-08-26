use std::fmt::{Display, Formatter, Result};

use crate::token::Token;

#[derive(Debug)]
pub enum Error {
    // Lexer Errors
    UnexpectedCharacter(char),
    // Parser Errors
    EmptyTokens,
    UnexpectedToken(TokenType, Token, usize),
    UnexpectedEndOfLine(char),
}

#[derive(Debug)]
pub enum TokenType {
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Lit,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::UnexpectedCharacter(c) => write!(f, "Found unexpected character: {}", c),
            Error::EmptyTokens => write!(f, "Expected at least 1 token, found 0",),
            Error::UnexpectedToken(exp, fnd, pos) => {
                write!(f, "Expected {}, found {} at pos {}", exp, fnd, pos)
            }
            Error::UnexpectedEndOfLine(c) => write!(f, "Expected '\\r\\n' or '\\n' found {}", c),
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Mul => write!(f, "Mul"),
            TokenType::Div => write!(f, "Div"),
            TokenType::Mod => write!(f, "Mod"),
            TokenType::Lit => write!(f, "Lit"),
        }
    }
}

impl std::error::Error for Error {}
