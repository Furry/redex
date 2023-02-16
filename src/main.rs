#![feature(associated_const_equality)]

use parser::{ parser::Parser, lexer };
use serde_json;

use crate::runtime::Runtime;

pub mod runtime;
pub mod debug;
#[cfg(test)]
pub mod tests;

fn main() {
    let input = 
    r#"
    print("Hello World!")
    print(" uwu")
    "#;
    let tokens = lexer::parse_tokens(input);
    let mut parser = Parser::new(tokens);
    let mut runtime = Runtime::new();
    runtime.link_std();

    let result = parser.parse();
    // let json = serde_json::to_string(&result).unwrap();
    // println!("{}", json);
    println!("{:#?}", result);

    runtime.evaluate(&result);
    // runtime.coredump();
}