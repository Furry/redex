use std::collections::VecDeque;

use crate::types::{token::Token, ast::{Node, MathNode}};

pub struct Parser {
    pub input: VecDeque<Token>,

    pos: usize, 
    len: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Parser {
            len: tokens.len(),
            input: VecDeque::from(tokens),
            pos: 0,
        }
    }
}

impl Parser {
    pub fn parse(&mut self, start: usize, stop: usize) {
        // Create a new slice of input with the start and stop positions.
        let mut input = self.input.clone();
        input.drain(..start);
        input.drain(stop..);

        // Create our types
        let mut nodes: VecDeque<Node> = VecDeque::new();
        let mut hold: Option<Node> = None;

        while let Some(token) = input.pop_front() {
            let kind = token.clone().token_type;

            if kind.is_math() {
                let node = Node::Math(Node::into_math(&token));
                if hold.is_some() {
                    let mut hold = hold.unwrap();
                    hold = Node::into_math(&token).left(hold);
                    nodes.push_back(hold);
                } else {
                    nodes.push_back(node);
                }
            }

            if kind.is_literal() {
                if hold.is_none() {
                    hold = Some(Node::from::<Token>(token))
                }
            }
        }
    }

    pub fn parse2(&mut self) {
        
    }

    pub fn next(&mut self) {
        return self.parse(self.pos, self.input.len() - 1);
    }
}