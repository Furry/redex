use lexer::instance::collector::{TokenTuple, Token};

use crate::modules::Branch;
use super::Operation;

// #[derive(Debug)]
// pub enum MathOperation {
//     Addition,
//     Subtraction,
//     Multiplication,
//     Division,
//     Modulo,
//     Exponentiation,
//     Pass
// }

// From a vec<token> construct an evaluatable branch.
pub fn construct_branch(tokens: Vec<TokenTuple>) -> Branch {
    let mut branch = Branch::new(super::Operation::Pass, String::new(), None);
    let mut current = &mut branch;

    for token in tokens {
        match token.0 {
            Token::Number => {
                current.left = token.1;
            },
            Token::Other => {
                let operation = match token.1.as_str() {
                    "+" => Operation::Addition,
                    "-" => Operation::Subtraction,
                    "*" => Operation::Multiplication,
                    "/" => Operation::Division,
                    _ => Operation::Unknown
                };
                current.operation = operation;
                current.right = Some(Box::new(Branch::new(Operation::Pass, String::new(), None)));
                current = current.right.as_mut().unwrap();
            },
            _ => {}
        }
    }

    branch
}