use parser::{ parser::Parser, lexer };
use serde_json;

fn main() {
    // let input = r#"let x = 33"#;
    let input = "(((33 + 44) + (69) + 33) + (55 * 24))";
    // let input2 = "(63)"; 
    let tokens = lexer::parse_tokens(input);
    let mut parser = Parser::new(tokens);

    let result = parser.parse();
    // Convert it into json

    let json = serde_json::to_string_pretty(&result).unwrap();
    println!("{}", json);
}