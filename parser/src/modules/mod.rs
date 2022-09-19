use std::{ops::{Generator, GeneratorState}, pin::Pin};

use lexer::instance::collector::{TokenGenerator, Token, TokenTuple};

#[derive(Clone, Debug)]
pub struct Branch {
    pub left: Option<Box<Branch>>,
    pub right: Option<Box<Branch>>,
    pub parent: Option<Box<Branch>>,
}

pub struct AST {
    branches: Vec<Branch>,
    gen: TokenGenerator,
}

impl AST {
    pub fn new(generator: TokenGenerator) -> Self {
        Self {
            branches: Vec::new(),
            gen: generator
        }
    }
}

// Handle navigating branches.
impl AST {
    
}

impl AST {
    pub fn parse(&mut self) {
        let mut branch = Branch { left: None, right: None, parent: None };
        let mut stack: Vec<Branch> = Vec::new();

        loop {
            match Pin::new(&mut self.gen).resume(()) {
                GeneratorState::Yielded(value) => {
                    
                },
                GeneratorState::Complete(_) => break
            }
        }
    }
}
