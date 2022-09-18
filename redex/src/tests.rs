use crate::modules;


#[test]
fn lex_expression() {
    let code = "2 / 2 * (2 - 2)";
    let mut ctx = modules::context::Context::new();
    let tokens = ctx.lowlevel.lex(code);
    let x = tokens.iter()
        .map(|t| t.clone().1)
        .collect::<String>();
    assert!(x == code)
}

#[test]
fn lex_function()  {
    let code =  r#"
    fn myFunction(arg1, arg2) {
        System::io::print()
    }
    "#;

    let mut ctx = modules::context::Context::new();
    let tokens = ctx.lowlevel.lex(code);
    let x = tokens.iter()
        .map(|t| t.clone().1)
        .collect::<String>();
    assert!(x == code);
}