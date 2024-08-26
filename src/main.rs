use lexer::Lexer;

mod error;
mod lexer;
mod token;

fn main() {
    let mut lexer = Lexer::from_file("./hello.lost");

    let tokens = lexer.tokenize().unwrap();

    println!("{:?}", tokens);
}
