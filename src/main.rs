#![feature(associated_const_equality)]
#![feature(box_into_inner)]

use std::io::Write;

use parser::{ parser::Parser, lexer };
use serde_json;

use crate::runtime::Runtime;

pub mod runtime;
pub mod debug;
pub mod cli;
pub mod examples;

pub fn execute<T: Into<String>>(input: T) {
    let input: String = input.into();

    let tokens = lexer::parse_tokens(&input);
    let mut parser = crate::Parser::new(tokens);
    let mut runtime = Runtime::new();
    runtime.link_std();

    let result = parser.parse();
    runtime.evaluate(&result);
}

fn main() {
    // Process the CLI, providing the menu.
    cli::parse_cli();
}