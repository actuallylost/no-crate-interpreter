use lexer::Lexer;
use parser::Parser;

mod error;
mod lexer;
mod parser;
mod token;

fn main() {
    let lexer = Lexer::from_path("./hello.lost");

    let _tokens = lexer.tokenize().unwrap();

    // println!("{:?}", tokens);

    let input = "aaaaaaaaaa";
    let parsed = Parser::parse(input);

    if parsed.is_ok() {
        println!("Sucessfully parsed: '{}'", input);
    } else {
        println!("{:?}", parsed.unwrap());
    }
}
