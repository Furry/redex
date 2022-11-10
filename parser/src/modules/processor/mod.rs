use std::{collections::VecDeque, ops::{GeneratorState, Generator}, pin::Pin};

use lexer::instance::collector::{TokenGenerator, Token};

use super::ast::{Program, self, Expression};

pub enum ExpressionType {
}

pub struct Rule {
    pub name: String,
    pub expression: ExpressionType,
}

pub struct Processor {
    pub rules: Vec<Rule>,
    gen: TokenGenerator,
    pub input: String,
}

impl Processor {
    pub fn new(input: &String) -> Self {
        Self {
            gen: lexer::instance::Instance::new().generator(input),
            input: input.to_string(),
            rules: Vec::new()
        }
    }
}

impl Processor {
    pub fn is_math(input: &String) -> bool {
        match input.as_str() {
            "+" | "-" | "*" | "/" | "%" | "^" => true,
            _ => false
        }
    }

    pub fn process(mut self) {
        let mut program = Program {
            meta: ast::ExpressionMeta {
                start: 0,
                end: self.input.len()
            },
            children: Vec::new()
        };

        let mut token_stack: VecDeque<Token> = VecDeque::new();
        let mut expression_stack: VecDeque<Expression> = VecDeque::new();
        let mut depth: usize = usize::try_from(0).unwrap();

        loop {
            match Pin::new(&mut self.gen).resume(()) {
                GeneratorState::Yielded(value) => {
                    // Destructure
                    let (token, literal, start, end) = value;
                    println!("{:?}", token);
                    match token {
                        Token::Whitespace => {},
                        _ => {
                            if Self::is_math(&literal) {
                                
                            }
                        }
                    }
                },
                GeneratorState::Complete(_) => break
            }
        }
        // println!("{:?}", stack)
    }
}