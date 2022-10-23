mod lexer;
mod parser;
mod token;

use lexer::Lexer;
use parser::Parser;
use std::env;
use std::fs;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();

    if args.len() != 1 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let source = fs::read_to_string(&args[0]).expect("Unable to read file");
    let mut lexer = Lexer::new(&source);
    let mut parser = Parser::new(&mut lexer);

    let ast = parser.parse();

    println!("{:#?}", ast);
}
