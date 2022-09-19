#![feature(generator_trait)]
use std::{ops::{Generator, GeneratorState}, pin::Pin};

pub mod modules;
#[cfg(test)]
pub mod tests;

fn main() {
    let code = r#"
let x = 53 - (64 + 22) + 35221 * 635.242;
let potato = "delicious~" // yum yum;
"#;
    // let code = "53 - (64 + 22) + 35221 * 635242 + \"hewwo~ uwu~\"";
    let mut ctx = modules::context::Context::new();
    let mut r = ctx.lowlevel.lexer.generator(code);

    // Iterate over the generator
    loop {
        match Pin::new(&mut r).resume(()) {
            GeneratorState::Yielded(value) => {
                println!("{:?}", value);
            },
            GeneratorState::Complete(_) => break
        }
    }
}
