#![feature(generator_trait)]
#![feature(trait_alias)]

use std::{ops::{Generator, GeneratorState}, pin::Pin, borrow::Cow};

use lexer::{instance::collector::{TokenGenerator, TokenTuple, Token}, strum::EnumProperty};
use structures::{Operation, AST::AST };
use structures::{Branchable, MaybeValue, BranchValue};

pub mod modules;
pub mod structures;

pub struct Parser {
    generator: TokenGenerator,
    tokens: Vec<TokenTuple>,
    index: usize,
    stack: Vec<Box<dyn Branchable>>,
}

// To traverse the list of tokens.
impl Parser {
    pub fn new(generator: TokenGenerator) -> Self {
        Self {
            tokens: Vec::new(),
            stack: Vec::new(),
            generator,
            index: 0
        }
    }

    pub fn next(&mut self) -> Option<TokenTuple> {
        // If the index is smaller than the length of the vector, return the token at the index from the vec.
        if self.index < self.tokens.len() {
            let token = self.tokens[self.index].clone();
            self.index += 1;
            return Some(token);
        } else {
            match Pin::new(&mut self.generator).resume(()) {
                GeneratorState::Yielded(value) => {
                    self.tokens.push(value.clone());
                    self.index += 1;
                    return Some(value);
                },
                GeneratorState::Complete(_) => return None
            }
        }
    }

    pub fn previous(&mut self) -> Option<TokenTuple> {
        if self.index == 0 {
            return None;
        } else {
            self.index -= 1;
            return Some(self.tokens[self.index].clone());
        }
    }

    pub fn until(&mut self, token: Token) -> Vec<TokenTuple> {
        let mut vec: Vec<TokenTuple> = Vec::new();
        loop {
            match self.next() {
                Some(value) => {
                    if value.0 == token {
                        return vec;
                    } else {
                        vec.push(value);
                    }
                },
                None => return vec
            }
        }
    }

    pub fn until_str(&mut self, string: &str) -> Vec<TokenTuple> {
        let mut vec: Vec<TokenTuple> = Vec::new();
        loop {
            match self.next() {
                Some(value) => {
                    if value.1 == string {
                        return vec;
                    } else {
                        vec.push(value);
                    }
                },
                None => return vec
            }
        }
    }
}

pub trait TokenVectorTraversable {
    fn until_str(&self, string: &str) -> Option<(usize, Vec<TokenTuple>)>;
}

impl TokenVectorTraversable for Vec<TokenTuple> {
    fn until_str(&self, s: &str) -> Option<(usize, Vec<TokenTuple>)> {
        let mut vec: Vec<TokenTuple> = Vec::new();
        for (index, value) in self.iter().enumerate() {
            if value.1 == s {
                return Some((index, vec));
            } else {
                vec.push(value.clone());
            }
        }
        return None;
    }
}

#[derive(PartialEq, Eq)]
enum HandlingState {
    Expression,
    Idle
}

// To handle outwards facing functions.
impl Parser {
    // pub fn parse_dep(&mut self) {
    //     let mut handling: HandlingState = HandlingState::Idle;

    //     while let Some(token) = self.next() {
    //         match token.0 {
    //             Token::Number => {
    //                 if handling == HandlingState::Idle {
    //                     handling = HandlingState::Expression;
    //                     let f = self.until(Token::EndOfLine)
    //                         .into_iter()
    //                         .filter(|p| p.0 != Token::Whitespace)
    //                         .collect::<Vec<TokenTuple>>();
    //                     walk_expression(token, f);
    //                 }
    //             },
    //             _ => ()
    //         }
    //     }
    // }

    // pub fn parse(&mut self) {
    //     let mut root = AST::new();


    //     while let Some(token) = self.next() {
            
    //     }
    // }
}