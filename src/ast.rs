#[derive(Debug, PartialEq)]
pub struct Ast(Vec<Stmt>);

impl Ast {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, val: Stmt) {
        self.0.push(val)
    }

    pub fn _pop(&mut self) -> Option<Stmt> {
        self.0.pop()
    }
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(Box<Expr>),
}

/// Defines all Expression variants
#[derive(Debug, PartialEq)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Lit(i32),
}
