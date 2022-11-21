#![feature(generator_trait)]
#![feature(trait_alias)]

use std::{ops::{Generator, GeneratorState}, pin::Pin, borrow::Cow};

use lexer::{instance::collector::{TokenGenerator, TokenTuple, Token}, strum::EnumProperty};
use modules::Branch;
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
    pub fn parse_dep(&mut self) {
        let mut handling: HandlingState = HandlingState::Idle;

        while let Some(token) = self.next() {
            match token.0 {
                Token::Number => {
                    if handling == HandlingState::Idle {
                        handling = HandlingState::Expression;
                        let f = self.until(Token::EndOfLine)
                            .into_iter()
                            .filter(|p| p.0 != Token::Whitespace)
                            .collect::<Vec<TokenTuple>>();
                        walk_expression(token, f);
                    }
                },
                _ => ()
            }
        }
    }

    pub fn parse(&mut self) {
        let mut root = AST::new();


        while let Some(token) = self.next() {
        }
    }
}

fn walk_expression(left: TokenTuple, right: Vec<TokenTuple>) -> Box<Branch> {
    // // Construct a tree recursively from the right side..
    // // let mut stack: Vec<Branch> = Vec::new();
    // let mut index = 0;
    
    // let mut operation: Operation = Operation::test(&left);
    // if operation == Operation::Unknown {
    //     if let Some(value) = right.get(index) {
    //         operation = Operation::test(&value);
    //     }
    //     if operation == Operation::Unknown {
    //         panic!("Invalid right hand expression!");
    //     }
    // }

    // let branch = &mut Branch::new(operation, left.1, None);
    // let mut new: Option<Box<Branch>> = None;
    // loop {
    //     if let Some(value) = right.get(index) {
    //         index += 1;
    //         let operation = Operation::test(&value);
    //         if operation == Operation::Unknown {
    //             continue;
    //         } else {
    //             new = Some(Box::new(Branch::new(operation, value.clone().1, None)));
    //             // Iterate over the tree and find the last branch.
    //             let mut current = &mut *branch;
    //             loop {
    //                 if let Some(ref mut value) = current.right {
    //                     current = value;
    //                 } else {
    //                     if (index + 1) == right.len() {
    //                         let mut n = new.unwrap().clone();
    //                         n.right = Some(Box::new(Branch::new(Operation::Unknown, right[index].clone().1, None)));
    //                         current.right = Some(n);
    //                     } else {
    //                         current.right = new;
    //                     }
    //                     break;
    //                 }
    //                 index += 1;
    //             }
    //         }
    //     } else {
    //         break;
    //     }
    // }

    let mut index = 0;
    let mut operation: Operation = Operation::test(&left);
    if operation == Operation::Unknown {
        if let Some(value) = right.get(index) {
            operation = Operation::test(&value);
            index += 1;
        }
        if operation == Operation::Unknown {
            panic!("Invalid right hand expression!");
        }
    }

    // println!("Vec: {:?}", right);
    // println!("Operation: {:?}", right.until_str(operation.get_str("Key").unwrap()));

    // loop {
        
    // }

    todo!()
}