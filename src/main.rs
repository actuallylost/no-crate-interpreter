use std::time::Instant;

use lexer::Lexer;
use parser::Parser;

use crate::ast::Execute;

mod ast;
mod error;
mod lexer;
mod parser;
mod token;

fn main() {
    let now = Instant::now();
    let lexer = Lexer::from_path("./hello.lost");

    let tokens = lexer.tokenize().unwrap();

    println!("Tokens: '{:?}'", tokens);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let elapsed_parse = now.elapsed();

    if ast.is_ok() {
        println!(
            "Parsed (in {:?}): '{:?}'",
            elapsed_parse,
            ast.as_ref().unwrap()
        );
        ast.unwrap().execute();
        let elapsed_exec = now.elapsed();
        println!("Took {:?}", elapsed_exec);
    } else {
        println!("{:?}", ast.unwrap());
    }
}
