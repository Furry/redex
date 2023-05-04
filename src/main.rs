#![feature(associated_const_equality)]

use std::io::Write;

use parser::{ parser::Parser, lexer };
use serde_json;

use crate::runtime::Runtime;

pub mod runtime;
pub mod debug;
#[cfg(test)]
pub mod tests;

/*
println("Enter text: ")
let x = readln()
let response = "you entered: " + x
println(response)
*/

fn main() {
    cli();
}

#[allow(dead_code)]
fn oneshot() {
    let input = 
    r#"
    print("hi");
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
    runtime.coredump();
}

fn cli() {
    // // Constantly ask for input to evaluate with > prompt.
    // let mut input = String::new();
    let mut runtime = Runtime::new();
    runtime.link_std();

    // loop {
    //     print!("> ");
    //     std::io::stdin().read_line(&mut input).unwrap();

    //     let tokens = lexer::parse_tokens(&input);
    //     let mut parser = Parser::new(tokens);
    //     let result = parser.parse();
    //     runtime.evaluate(&result);

    //     input.clear();
    // }

    loop {
        print!("redex> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "exit" {
                    break;
                } else {
                    let ast = Parser::new(
                        lexer::parse_tokens(&input)
                    ).parse();

                    runtime.evaluate(&ast);
                }
            }
            Err(error) => println!("Error reading input: {}", error),
        }
    }
}