use parser::{ parser::Parser, lexer };
use serde_json;

fn main() {
    let input = r#"let x = 33"#;
    let input2 = "33 + (4 * (67 / 3))";
    let tokens = lexer::parse_tokens(input2);
    let mut parser = Parser::new(tokens);

    let result = parser.parse();
    // Convert it into json

    let json = serde_json::to_string(&result).unwrap();
    println!("{}", json);
}