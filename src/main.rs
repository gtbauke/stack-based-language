mod compiler;
mod lexer;
mod parser;
mod runtime;
mod token;

use compiler::Compiler;
use lexer::Lexer;
use parser::Parser;
use std::{env, fs};

use crate::{compiler::resolver::Resolver, runtime::Interpreter};

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();

    if args.len() != 1 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let source = fs::read_to_string(&args[0]).expect("Unable to read file");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.lex();

    // println!("{:#?}", &tokens);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut resolver = Resolver::new();
    let program = match resolver.resolve(ast.clone()) {
        Ok(program) => program,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    let mut compiler = Compiler::new(program);
    let program = compiler.compile(ast).expect("Unable to compile program");

    println!("{:#?}", &program);

    let mut interpreter = Interpreter::new(program.clone());
    let result = interpreter
        .interpret()
        .expect("Unable to interpret program");

    println!("{:#?}", result);
}
