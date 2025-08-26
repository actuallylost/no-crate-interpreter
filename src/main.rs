use std::time::Instant;

use lexer::Lexer;
use parser::Parser;

mod ast;
mod error;
mod lexer;
mod parser;
mod token;

fn main() {
    let lexer = Lexer::from_path("./hello.lost");

    let tokens = lexer.tokenize().unwrap();

    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    let now = Instant::now();
    let ast = parser.parse();
    let elapsed = now.elapsed();

    if ast.is_ok() {
        println!("Parsed (in {:?}): '{:?}'", elapsed, ast.unwrap());
    } else {
        println!("{:?}", ast.unwrap());
    }
}
