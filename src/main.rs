#![feature(associated_const_equality)]
#![feature(box_into_inner)]

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
    // cli();
    oneshot();
}

#[allow(dead_code)]
fn oneshot() {
    let input = 
    // r#"
    // let i = 0
    // while (true) {
    //     let a = i % 3
    //     let b = i % 5
    //     print(i)
    //     if (a == 0) {
    //         print("Fizz")
    //     }
    //     if (b == 0) {
    //         print("Buzz")
    //     }
    //     println("")
    //     let i = i + 1
    // }
    // "#;
    r#"
    let y = net_get("https://google.com")
    println(y)
    "#;
    let tokens = lexer::parse_tokens(input);
    let mut parser = Parser::new(tokens);
    let mut runtime = Runtime::new();
    runtime.link_std();

    let result = parser.parse();
    let json = serde_json::to_string(&result).unwrap();
    // println!("{}", json);
    // println!("{:#?}", result);

    runtime.evaluate(&result);
    // runtime.coredump();
}

fn cli() {
    let mut runtime = Runtime::new();
    runtime.link_std();

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