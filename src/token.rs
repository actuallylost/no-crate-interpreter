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
    Equals,
    Lit(i32),
    Ident(String),
}
