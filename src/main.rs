use parser::{ parser::Parser, lexer };
use serde_json;

pub mod runtime;
#[cfg(test)]
pub mod tests;

fn main_() {
    // let input = r#"let x = (((33 + 44) + (69) + 33) + (55 * 24))"#;
    // let input = "(((33 + 44) + (69) + 33) + (55 * 24))";
    // let input2 = "(63)"; 
    let input = r#"let x = 55"#;
    let function = r#"fn myFunction() { return 55 } let x = 33"#;
    let tokens = lexer::parse_tokens(function);
    let mut parser = Parser::new(tokens);

    let result = parser.parse();
    // Convert it into json

    
    let json = serde_json::to_string_pretty(&result).unwrap();
    println!("{}", json);

    // let mut runtime = runtime::Runtime::new();
    // runtime.evaluate(&result);
    // runtime.coredump();
    // println!("{}", x.to_string())
}

fn main() {
    let input = r#"
    fn myFunctionOne() {
        let x = fn myFunctionTwo() {
            let y = "hello"
            let z = "world"
            let math = (((33 + 44) + (69) + 33) + (55 * 24))
            return y + z + math
        }
        return 44
    }
    "#;
    let tokens = lexer::parse_tokens(input);
    let mut parser = Parser::new(tokens);

    let result = parser.parse();
    let json = serde_json::to_string(&result).unwrap();
    println!("{}", json);
}