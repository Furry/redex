use parser::{ parser::Parser, lexer };
use serde_json;

use crate::runtime::Runtime;

pub mod runtime;
#[cfg(test)]
pub mod tests;

fn main() {
    let input = 
    r#"
    let x = 34 + 2 * 70;
    let y = 55;
    let z = 44;
    "#;
    let tokens = lexer::parse_tokens(input);
    let mut parser = Parser::new(tokens);
    let mut runtime = Runtime::new();

    let result = parser.parse();
    // let json = serde_json::to_string(&result).unwrap();
    // println!("{}", json);

    runtime.evaluate(&result);
    runtime.coredump();
}