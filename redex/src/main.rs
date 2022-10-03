#![feature(generator_trait)]
use std::{ops::{Generator, GeneratorState}, pin::Pin};

use parser::structures::math::construct_branch;

pub mod modules;
#[cfg(test)]
pub mod tests;

fn main() {
    let code = r#"5 + 7 * 4;"#;
    // let code = "53 - (64 + 22) + 35221 * 635242 + \"hewwo~ uwu~\"";
    let mut ctx = modules::context::Context::new();
    let r = ctx.lowlevel.lexer.generator(code);
    ctx.lowlevel.parse(r);
    // Iterate over the generator
    // loop {
    //     match Pin::new(&mut r).resume(()) {
    //         GeneratorState::Yielded(value) => {
    //             println!("{:?}", value);
    //         },
    //         GeneratorState::Complete(_) => break
    //     }
    // }
}
