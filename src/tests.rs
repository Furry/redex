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
    }

    print(true)"#;

    let tokens = lexer::parse_tokens(input);
    let mut parser = Parser::new(tokens);

    let result = parser.parse();
    println!("{:?}", result);
}