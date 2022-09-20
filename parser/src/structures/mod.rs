use std::fmt::Debug;

use lexer::instance::collector::{TokenTuple, Token};

use self::math::MathOperation;

pub mod math;

pub trait BranchValue: Branchable + MaybeValue + Debug {}

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Define,
    Assign,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Unknown
}

impl Operation {
    pub fn test(token: &TokenTuple) -> Operation {
        match token.1.as_str() {
            "+" => Operation::Addition,
            "-" => Operation::Subtraction,
            "*" => Operation::Multiplication,
            "/" => Operation::Division,
            "=" => Operation::Assign,
            _ => Operation::Unknown
        }
    } 
}

pub trait Branchable {
    fn evaluate(&self) -> Box<dyn MaybeValue>;
}

impl Debug for dyn Branchable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Branchable")
    }
}

pub trait MaybeValue {
    fn try_string(&self) -> Option<String>;
    fn try_number(&self) -> Option<f64>;
    fn try_bool(&self) -> Option<bool>;
}