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
/// Add -> Lit + Add | Lit
///
/// Sub -> Add - Sub
/// Sub -> Add
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
        if self.tokens.len() == 0 {
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

    // Sub -> Add - Sub
    // Sub -> Add
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

    // Add -> Lit + Add | Lit
    /// Returns an `Expr::Add` if parsing is successful
    fn add(&mut self) -> Result<Expr, Error> {
        let lit = self.lit()?;
        // println!("Add (lit): {:?}, {}", lit, self.state.cursor);

        match self.advance_if(|tkn| matches!(tkn, Token::Plus)) {
            Some(t) => {
                // println!("Add (some): {t}, {}", self.state.cursor);
                Ok(Expr::Add(Box::new(lit), Box::new(self.add().unwrap())))
            }
            None => {
                // println!("Add (none): {:?}, {}", lit, self.state.cursor);
                Ok(lit)
            }
        }
    }

    /// Returns an `Expr::Mul` if parsing is successful
    fn _mul(a: i32, b: i32) -> Result<Expr, Error> {
        Ok(Expr::Mul(Box::new(Expr::Lit(a)), Box::new(Expr::Lit(b))))
    }

    /// Returns an `Expr::Div` if parsing is successful
    fn _div(a: i32, b: i32) -> Result<Expr, Error> {
        Ok(Expr::Div(Box::new(Expr::Lit(a)), Box::new(Expr::Lit(b))))
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
}
