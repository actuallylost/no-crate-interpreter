use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
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
            Token::Lit(n) => write!(f, "{}", n),
            Token::Ident(s) => write!(f, "{}", s),
        }
    }
}
