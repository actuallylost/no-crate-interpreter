#[derive(Debug, PartialEq)]
pub struct Ast(Vec<Stmt>);

pub trait Execute {
    type Output;

    fn execute(&self) -> Self::Output;
}

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

impl Execute for Ast {
    type Output = ();

    fn execute(&self) -> Self::Output {
        match self.0.iter().next() {
            Some(s) => match s {
                Stmt::Expr(expr) => {
                    println!("Ast: {:?}", expr);
                    println!("Result: {:?}", expr.execute());
                }
            },
            None => todo!(),
        }
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
    Unary(Box<Expr>),
    Lit(i32),
}

impl Execute for Expr {
    type Output = i32;

    fn execute(&self) -> Self::Output {
        match self {
            Expr::Add(a, b) => {
                // println!("Expr: {:?} + {:?}", a, b);
                let exec_a = a.execute();
                let exec_b = b.execute();
                // println!("i32: {:?} + {:?}", exec_a, exec_b);
                exec_a + exec_b
            }
            Expr::Sub(a, b) => {
                // println!("Expr: {:?} - {:?}", a, b);
                let exec_a = a.execute();
                let exec_b = b.execute();
                // println!("i32: {:?} - {:?}", exec_a, exec_b);
                exec_a - exec_b
            }
            Expr::Mul(a, b) => {
                // println!("Expr: {:?} * {:?}", a, b);
                let exec_a = a.execute();
                let exec_b = b.execute();
                // println!("i32: {:?} * {:?}", exec_a, exec_b);
                exec_a * exec_b
            }
            Expr::Div(a, b) => {
                // println!("Expr: {:?} / {:?}", a, b);
                let exec_a = a.execute();
                let exec_b = b.execute();
                // println!("i32: {:?} / {:?}", exec_a, exec_b);
                exec_a / exec_b
            }
            Expr::Unary(expr) => todo!(),
            Expr::Lit(n) => *n,
        }
    }
}
