#[derive(Debug)]
pub enum Token {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Assign,
    Lit(String),
    Ident(String),
}
