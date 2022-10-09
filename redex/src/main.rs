#![feature(generator_trait)]
use std::{ops::{Generator, GeneratorState}, pin::Pin};

use lexer::instance::collector::Collector;
use modules::context::lowlevel;
use parser::structures::math::construct_branch;

pub mod modules;
#[cfg(test)]
pub mod tests;

fn main() {
    // let code = r#"(5 + 7) * 4 + (33 * 2);"#;
    // let code = "53 - (64 + 22) + 35221 * 635242 + \"hewwo~ uwu~\"";
    // let mut ctx = modules::context::Context::new();
    // let r = ctx.lowlevel.lexer.generator(code);
    // ctx.lowlevel.parse(r);

    // Iterate over the generator
    // loop {
    //     match Pin::new(&mut r).resume(()) {
    //         GeneratorState::Yielded(value) => {
    //             println!("{:?}", value);
    //         },
    //         GeneratorState::Complete(_) => break
    //     }
    // }


    let code = "5 + 7 + 3";
    let mut ctx = modules::context::Context::new();
    let r = ctx.lowlevel.lexer.generator(code);
    let v = Collector::vec(r);
    let b = construct_branch(v);
    let n = lowlevel::LowLevel::evaluate(b);
    println!("{:?}", n);
}
