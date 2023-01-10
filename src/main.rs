use parser::{ parser::Parser, lexer };

fn main() {
    let input = r#"let x = 33"#;
    let input2 = "33 + 4 * 67";
    let tokens = lexer::parse_tokens(input2);
    let mut parser = Parser::new(tokens);

    dbg!(parser.next());
    // test();
}

/*
{
    
}
*/