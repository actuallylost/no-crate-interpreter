use crate::{
    ast::{Ast, Expr, Stmt},
    error::{Error, TokenType},
    token::Token,
};

pub struct ParserState {
    cursor: usize,
}

impl ParserState {
    pub fn new(cursor: usize) -> Self {
        Self { cursor }
    }
}

/// # Grammar
/// ```
/// S -> Sub? EOF
///
/// Lit -> [0-9]+
///
/// Div -> Lit / Div | Lit
///
/// Mul -> Div * Mul | Div
///
/// Add -> Mul + Add | Mul
///
/// Sub -> Add - Sub | Add
/// ```
pub struct Parser {
    tokens: Vec<Token>,
    state: ParserState,
}

impl Parser {
    /// Creates a `Parser` instance
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            state: ParserState::new(0),
        }
    }

    /// Parses the tokenized source code, and returns an `Ast`
    pub fn parse(&mut self) -> Result<Ast, Error> {
        if self.tokens.len() == 0 || *self.current().unwrap() == Token::EOF {
            return Err(Error::EmptyTokens);
        }

        let mut ast = Ast::new();
        let expr = self.expr()?;
        ast.push(expr);

        Ok(ast)
    }

    /// Peeks at the next token.
    pub fn _peek(&self) -> Option<&Token> {
        self.tokens.get(self.state.cursor + 1)
    }

    /// Borrows the current token.
    pub fn current(&self) -> Option<&Token> {
        self.tokens.get(self.state.cursor)
    }

    /// Consumes the current token, and updates the cursor.
    pub fn advance(&mut self) -> Option<Token> {
        let token = self.current().map(|t| t.to_owned());
        self.state.cursor += 1;
        token
    }

    // Advance the cursor if the closure returns true
    pub fn advance_if<F>(&mut self, f: F) -> Option<Token>
    where
        F: FnOnce(Token) -> bool,
    {
        let current = self.current()?;

        if f(current.clone()) {
            self.advance()
        } else {
            None
        }
    }

    // Backtracks, updates the cursor and returns the preceding token.
    pub fn backtrack(&mut self) -> Option<Token> {
        self.state.cursor -= 1;
        self.current().map(|t| t.to_owned())
    }

    /// Return a Stmt::Expr if parsing is successful
    fn expr(&mut self) -> Result<Stmt, Error> {
        self.sub().map(|expr| Stmt::Expr(Box::new(expr)))
    }

    // Div -> Mul / Div | Mul
    /// Returns an `Expr::Div` if parsing is successful
    fn div(&mut self) -> Result<Expr, Error> {
        let lit = self.lit()?;
        // println!("Div (lit): {:?}, {}", lit, self.state.cursor);

        match self.advance_if(|tkn| matches!(tkn, Token::Div)) {
            Some(t) => {
                // println!("Div (some): {:?}, {}", t, self.state.cursor);
                Ok(Expr::Div(Box::new(lit), Box::new(self.div().unwrap())))
            }
            None => {
                // println!("Div (none): {:?}, {}", t, self.state.cursor);
                Ok(lit)
            }
        }
    }

    // Mul -> Div * Mul | Div
    /// Returns an `Expr::Mul` if parsing is successful
    fn mul(&mut self) -> Result<Expr, Error> {
        let div = self.div()?;
        // println!("Mul (div): {:?}, {}", div, self.state.cursor);

        match self.advance_if(|tkn| matches!(tkn, Token::Mul)) {
            Some(t) => {
                // println!("Mul (some): {:?}, {}", t, self.state.cursor);
                Ok(Expr::Mul(Box::new(div), Box::new(self.mul().unwrap())))
            }
            None => {
                // println!("Mul (none): {:?}, {}", t, self.state.cursor);
                Ok(div)
            }
        }
    }

    // Sub -> Add - Sub | Add
    /// Returns an `Expr::Sub` if parsing is successful
    fn sub(&mut self) -> Result<Expr, Error> {
        let add = self.add()?;
        // println!("Sub (add): {:?}, {}", add, self.state.cursor);

        match self.advance_if(|tkn| matches!(tkn, Token::Minus)) {
            Some(t) => {
                // println!("Sub (some): {:?}, {}", t, self.state.cursor);
                Ok(Expr::Sub(Box::new(add), Box::new(self.sub().unwrap())))
            }
            None => {
                // println!("Sub (none): {:?}, {}", add, self.state.cursor);
                Ok(add)
            }
        }
    }

    // Add -> Mul + Add | Mul
    /// Returns an `Expr::Add` if parsing is successful
    fn add(&mut self) -> Result<Expr, Error> {
        let mul = self.mul()?;
        // println!("Add (mul): {:?}, {}", mul, self.state.cursor);

        match self.advance_if(|tkn| matches!(tkn, Token::Plus)) {
            Some(t) => {
                // println!("Add (some): {t}, {}", self.state.cursor);
                Ok(Expr::Add(Box::new(mul), Box::new(self.add().unwrap())))
            }
            None => {
                // println!("Add (none): {:?}, {}", mul, self.state.cursor);
                Ok(mul)
            }
        }
    }

    // Lit -> [0-9]+
    /// Returns an `Expr::Lit` if parsing is successful
    fn lit(&mut self) -> Result<Expr, Error> {
        match self.advance_if(|tkn| matches!(tkn, Token::Lit(_))) {
            Some(Token::Lit(n)) => {
                // println!("Lit (some): {n}, {}", self.state.cursor);
                Ok(Expr::Lit(n))
            }
            None => {
                let curr = self.current().unwrap().to_owned();
                // println!("Lit (none): {:?}, {}", curr, self.state.cursor);
                self.backtrack();
                Err(Error::UnexpectedToken(
                    TokenType::Lit,
                    curr,
                    self.state.cursor,
                ))
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_add() {
        let tokens = vec![Token::Lit(10), Token::Plus, Token::Lit(273), Token::EOF];
        let mut parser = Parser::new(tokens);
        let actual = parser.parse().unwrap();

        let mut expected = Ast::new();
        expected.push(Stmt::Expr(Box::new(Expr::Add(
            Box::new(Expr::Lit(10)),
            Box::new(Expr::Lit(273)),
        ))));

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_add() {
        let tokens = vec![
            Token::Lit(10),
            Token::Plus,
            Token::Lit(273),
            Token::Plus,
            Token::Lit(19),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let actual = parser.parse().unwrap();

        let mut expected = Ast::new();
        expected.push(Stmt::Expr(Box::new(Expr::Add(
            Box::new(Expr::Lit(10)),
            Box::new(Expr::Add(Box::new(Expr::Lit(273)), Box::new(Expr::Lit(19)))),
        ))));

        assert_eq!(expected, actual);
    }

    #[test]
    fn single_sub() {
        let tokens = vec![Token::Lit(10), Token::Minus, Token::Lit(273), Token::EOF];
        let mut parser = Parser::new(tokens);
        let actual = parser.parse().unwrap();

        let mut expected = Ast::new();
        expected.push(Stmt::Expr(Box::new(Expr::Sub(
            Box::new(Expr::Lit(10)),
            Box::new(Expr::Lit(273)),
        ))));

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_sub() {
        let tokens = vec![
            Token::Lit(10),
            Token::Minus,
            Token::Lit(273),
            Token::Minus,
            Token::Lit(19),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let actual = parser.parse().unwrap();

        let mut expected = Ast::new();
        expected.push(Stmt::Expr(Box::new(Expr::Sub(
            Box::new(Expr::Lit(10)),
            Box::new(Expr::Sub(Box::new(Expr::Lit(273)), Box::new(Expr::Lit(19)))),
        ))));

        assert_eq!(expected, actual);
    }

    #[test]
    fn single_mul() {
        let tokens = vec![Token::Lit(10), Token::Mul, Token::Lit(273), Token::EOF];
        let mut parser = Parser::new(tokens);
        let actual = parser.parse().unwrap();

        let mut expected = Ast::new();
        expected.push(Stmt::Expr(Box::new(Expr::Mul(
            Box::new(Expr::Lit(10)),
            Box::new(Expr::Lit(273)),
        ))));

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_mul() {
        let tokens = vec![
            Token::Lit(10),
            Token::Mul,
            Token::Lit(273),
            Token::Mul,
            Token::Lit(19),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let actual = parser.parse().unwrap();

        let mut expected = Ast::new();
        expected.push(Stmt::Expr(Box::new(Expr::Mul(
            Box::new(Expr::Lit(10)),
            Box::new(Expr::Mul(Box::new(Expr::Lit(273)), Box::new(Expr::Lit(19)))),
        ))));

        assert_eq!(expected, actual);
    }

    #[test]
    fn single_div() {
        let tokens = vec![Token::Lit(10), Token::Div, Token::Lit(273), Token::EOF];
        let mut parser = Parser::new(tokens);
        let actual = parser.parse().unwrap();

        let mut expected = Ast::new();
        expected.push(Stmt::Expr(Box::new(Expr::Div(
            Box::new(Expr::Lit(10)),
            Box::new(Expr::Lit(273)),
        ))));

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_div() {
        let tokens = vec![
            Token::Lit(10),
            Token::Div,
            Token::Lit(273),
            Token::Div,
            Token::Lit(19),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let actual = parser.parse().unwrap();

        let mut expected = Ast::new();
        expected.push(Stmt::Expr(Box::new(Expr::Div(
            Box::new(Expr::Lit(10)),
            Box::new(Expr::Div(Box::new(Expr::Lit(273)), Box::new(Expr::Lit(19)))),
        ))));

        assert_eq!(expected, actual);
    }

    #[test]
    fn combined_add_then_sub() {
        let tokens = vec![
            Token::Lit(372),
            Token::Plus,
            Token::Lit(49),
            Token::Minus,
            Token::Lit(130),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let actual = parser.parse().unwrap();

        let mut expected = Ast::new();
        expected.push(Stmt::Expr(Box::new(Expr::Sub(
            Box::new(Expr::Add(Box::new(Expr::Lit(372)), Box::new(Expr::Lit(49)))),
            Box::new(Expr::Lit(130)),
        ))));

        assert_eq!(expected, actual);
    }

    #[test]
    fn combined_sub_then_add() {
        let tokens = vec![
            Token::Lit(372),
            Token::Minus,
            Token::Lit(49),
            Token::Plus,
            Token::Lit(130),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let actual = parser.parse().unwrap();

        let mut expected = Ast::new();
        expected.push(Stmt::Expr(Box::new(Expr::Sub(
            Box::new(Expr::Lit(372)),
            Box::new(Expr::Add(Box::new(Expr::Lit(49)), Box::new(Expr::Lit(130)))),
        ))));

        assert_eq!(expected, actual);
    }

    #[test]
    fn combined_mul_then_div() {
        let tokens = vec![
            Token::Lit(372),
            Token::Mul,
            Token::Lit(49),
            Token::Div,
            Token::Lit(130),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let actual = parser.parse().unwrap();

        let mut expected = Ast::new();
        expected.push(Stmt::Expr(Box::new(Expr::Mul(
            Box::new(Expr::Lit(372)),
            Box::new(Expr::Div(Box::new(Expr::Lit(49)), Box::new(Expr::Lit(130)))),
        ))));

        assert_eq!(expected, actual);
    }

    #[test]
    fn combined_div_then_mul() {
        let tokens = vec![
            Token::Lit(372),
            Token::Div,
            Token::Lit(49),
            Token::Mul,
            Token::Lit(130),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let actual = parser.parse().unwrap();

        let mut expected = Ast::new();
        expected.push(Stmt::Expr(Box::new(Expr::Mul(
            Box::new(Expr::Div(Box::new(Expr::Lit(372)), Box::new(Expr::Lit(49)))),
            Box::new(Expr::Lit(130)),
        ))));

        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn single_add_missing_first_num() {
        let tokens = vec![Token::Plus, Token::Lit(139), Token::EOF];
        let mut parser = Parser::new(tokens);
        // this should panic
        parser.parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn single_add_missing_second_num() {
        let tokens = vec![Token::Lit(32), Token::Plus, Token::EOF];
        let mut parser = Parser::new(tokens);
        // this should panic
        parser.parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn single_sub_missing_first_num() {
        let tokens = vec![Token::Minus, Token::Lit(139), Token::EOF];
        let mut parser = Parser::new(tokens);
        // this should panic
        parser.parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn single_sub_missing_second_num() {
        let tokens = vec![Token::Lit(32), Token::Minus, Token::EOF];
        let mut parser = Parser::new(tokens);
        // this should panic
        parser.parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn single_mul_missing_first_num() {
        let tokens = vec![Token::Mul, Token::Lit(139), Token::EOF];
        let mut parser = Parser::new(tokens);
        // this should panic
        parser.parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn single_mul_missing_second_num() {
        let tokens = vec![Token::Lit(32), Token::Mul, Token::EOF];
        let mut parser = Parser::new(tokens);
        // this should panic
        parser.parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn single_div_missing_first_num() {
        let tokens = vec![Token::Div, Token::Lit(139), Token::EOF];
        let mut parser = Parser::new(tokens);
        // this should panic
        parser.parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn single_div_missing_second_num() {
        let tokens = vec![Token::Lit(32), Token::Div, Token::EOF];
        let mut parser = Parser::new(tokens);
        // this should panic
        parser.parse().unwrap();
    }
}
