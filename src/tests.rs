use parser::{
    parser::Parser,
    lexer
};

#[test]
fn parse_function() {
    let input = "fn myFunction(input) { return 55 }";
    let tokens = lexer::parse_tokens(input);
    let mut parser = Parser::new(tokens);

    let result = parser.parse();
    println!("{:?}", result);
}

#[test]
fn parse_definition() {
    let input = "let x = 55";
    let tokens = lexer::parse_tokens(input);
    let mut parser = Parser::new(tokens);

    let result = parser.parse();
    println!("{:?}", result);
}

#[test]
fn conditional_flow() {
    let input = 
    r#"if (true) {
        let x = 55
    } else {
        let y = 44
    }
    
    if (false) {
        let pineapple = "bad"
    }"#;

    let tokens = lexer::parse_tokens(input);
    let mut parser = Parser::new(tokens);

    let result = parser.parse();
    println!("{:?}", result);
}

#[test]
fn nested_functions() {
    let input = 
    r#"fn myFunctionOne() {
        let x = fn myFunctionTwo() {
            let y = "hello"
            let z = "world"
            let math = (((33 + 44) + (69) + 33) + (55 * 24))
            return y + z + math
        }
        return 44
    }"#;
    let tokens = lexer::parse_tokens(input);
    let mut parser = Parser::new(tokens);

    let result = parser.parse();
    println!("{:?}", result);
}