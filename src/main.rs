use parser::types::token::{ parse_tokens, test};

fn main() {
    let input = r#"let x = 33"#;
    let tokens = parse_tokens(input);
    println!("{:?}", tokens)
    // test();
}
