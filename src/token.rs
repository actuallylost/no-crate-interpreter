use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Equals,
    Lit(i32),
    Ident(String),
    LF,
    CRLF,
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Mul => write!(f, "*"),
            Token::Div => write!(f, "/"),
            Token::Mod => write!(f, "%"),
            Token::Equals => write!(f, "="),
            Token::Lit(n) => write!(f, "Lit({})", n),
            Token::Ident(s) => write!(f, "Ident({})", s),
            Token::LF => write!(f, "\\n"),
            Token::CRLF => write!(f, "\\r\\n"),
            Token::EOF => write!(f, "EOF"),
        }
    }
}
