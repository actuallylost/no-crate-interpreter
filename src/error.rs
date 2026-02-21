use std::fmt::{Display, Formatter, Result};

use crate::token::{Token, TokenType};

#[derive(Debug)]
pub enum Error {
    // Lexer Errors
    UnexpectedCharacter(char),
    // Parser Errors
    EmptyTokens,
    UnexpectedToken(TokenType, Token, usize),
    UnexpectedEndOfLine(char),
    // Runtime Errors
    DivideByZero,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::UnexpectedCharacter(c) => write!(f, "Found unexpected character: {}", c),
            Error::EmptyTokens => write!(f, "Expected at least 1 token, found 0",),
            Error::UnexpectedToken(exp, act, pos) => {
                write!(f, "Expected {}, found {} at pos {}", exp, act, pos)
            }
            Error::UnexpectedEndOfLine(c) => write!(f, "Expected '\\r\\n' or '\\n' found {}", c),
            Error::DivideByZero => write!(f, "Cannot divide by zero"),
        }
    }
}

impl std::error::Error for Error {}
