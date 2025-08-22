use crate::{error::Error, parser::ParserState};

#[derive(Debug)]
pub struct Ast {}

impl Ast {
    pub fn new() -> Self {
        todo!();
    }
}

/// Defines all Expression variants
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Lit(i32),
}
