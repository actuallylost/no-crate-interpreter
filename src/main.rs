use lexer::Lexer;

mod error;
mod grammar;
mod lexer;
mod parser;
mod token;

fn main() {
    let lexer = Lexer::from_path("./hello.lost");

    let tokens = lexer.tokenize().unwrap();

    println!("{:?}", tokens);
}
