use std::fmt::Debug;

use lexer::{instance::collector::{TokenTuple, Token}, strum::EnumProperty};

pub mod math;
pub mod AST;
pub trait BranchValue: Branchable + MaybeValue + Debug {}

#[derive(Debug, PartialEq, Eq, Clone, EnumProperty)]
pub enum Operation {
    #[strum(props(Key = "let"))]
    Define,
    #[strum(props(Key = "="))]
    Assign,
    #[strum(props(Key = "+"))]
    Addition,
    #[strum(props(Key = "-"))]
    Subtraction,
    #[strum(props(Key = "*"))]
    Multiplication,
    #[strum(props(Key = "/"))]
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