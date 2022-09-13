use std::f32::consts::E;

use lexer::instance::collector::Token;
use lexer::strum::IntoEnumIterator;

use crate::modules;

#[test]
fn lex_equation() {
    let code = "3 + 4";
    let mut ctx = modules::context::Context::new();
    let tokens = ctx.lowlevel.lex(code);
    let x = tokens.iter()
        .map(|t| t.clone().1)
        .collect::<String>();
        
    println!("{:?}", x);
}