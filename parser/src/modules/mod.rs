use std::{ops::{Generator, GeneratorState}, pin::Pin, fmt::Debug};

use lexer::instance::collector::{TokenGenerator, Token, TokenTuple};

use crate::structures::{BranchValue, Branchable, MaybeValue, Operation};

#[derive(Debug, Clone)]
pub struct Branch {
    pub operation: Operation,
    pub left: String,
    pub right: Option<Box<Branch>>
}

impl Branch {
    pub fn new(operation: Operation, left: String, right: Option<Box<Branch>>) -> Self {
        Self {
            operation,
            left,
            right
        }
    }

    pub fn get_right(&self) -> Option<&Branch> {
        // Traverse all the way down the right side of the tree to get to the bottom. Return the reference to the node.
        let mut branch = self;
        loop {
            match branch.right {
                Some(ref right) => branch = right,
                None => return Some(branch)
            }
        }
    }

    pub fn set_right(self, new: Option<Box<Branch>>) {
        // Traverse all the way down the right side of the tree to get to the bottom. Set the right side of the node to the new branch.
        let mut branch = Box::new(self);

        loop {
            match branch.right {
                Some(right) => branch = right,
                None => {
                    break;
                }
            }
        }

        branch.right = new;
    }
}