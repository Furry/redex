use std::{ops::GeneratorState, ops::Generator, collections::VecDeque, pin::Pin};

use lexer::instance::collector::{TokenGenerator, Token};
use parser::modules::{ast::{Program, self, Expression}, processor::Processor};
pub struct LowLevel {
    pub lexer: lexer::instance::Instance
}

impl LowLevel {
    pub fn new() -> Self {
        Self { 
            lexer: 
                lexer::instance::Instance::new()
        }
    }
    
    
    // pub fn parse(&self, input: &String) -> TokenGenerator {
    //     return self.lexer.generator(input);
    // }

    pub fn process(&mut self, input: &String) {
        Processor::new(input).process();
    }
}