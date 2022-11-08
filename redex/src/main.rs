#![feature(generator_trait)]
use std::{ops::{Generator, GeneratorState}, pin::Pin};

use lexer::instance::collector::Collector;
use parser::modules::ast;
use modules::context::lowlevel;

pub mod modules;
#[cfg(test)]
pub mod tests;

fn main() {

    let x = ast::MathExpression {

    }
    println!("{}", x.unwrap())
}
