use std::{collections::HashMap, borrow::{BorrowMut, Borrow}};

use parser::types::ast::{Expression, LiteralExpression};

use self::primitives::{ VariableStorage };

pub mod primitives;
pub mod errors;

pub struct Runtime {
    pub variables: HashMap<String, primitives::VariableStorage>,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            variables: HashMap::new(),
        }
    }
}   

// Debug Functions
impl Runtime {
    pub fn coredump(&self) {
        for (key, value) in &self.variables {
            println!("{}: {:?}", key, value);
        }
    }

    fn assign(&mut self, key: String, value: VariableStorage) {
        // Check if the key already exists
        if self.variables.contains_key(&key) {
            panic!("Variable already exists");
        } else {
            // Insert the value
            self.variables.insert(key, value);
        }
    }

    fn pull<
        StdConversions, T: std::fmt::Debug + std::clone::Clone
    >(&self, key: &String) -> VariableStorage {
        // Check if the key exists
        if self.variables.contains_key(key) {
            // Return the value
            self.variables.get(key).unwrap().clone()
        } else {
            panic!("Request to unscoped variable");
        }
    }
}

impl Runtime {
    pub fn evaluate(&mut self, expression: &Expression) -> Option<VariableStorage> {
        match expression {
            Expression::Literal(literal) => {
                // Create a variable storage
                match literal.which {
                    parser::types::ast::LiteralType::String => {
                        // Create a variable storage
                        let storage = VariableStorage::String(
                            primitives::CompoundString::from(
                                literal.raw.clone()
                            )
                        );
                        // Return the storage
                        Some(storage)
                    },
                    parser::types::ast::LiteralType::Integer => {
                        // Create a variable storage
                        let storage = VariableStorage::Integer(
                            primitives::Integer::from(
                                literal.raw.parse::<i64>().unwrap()
                            )
                        );
                        // Return the storage
                        Some(storage)
                    },
                    parser::types::ast::LiteralType::Boolean => {
                        // Create a variable storage
                        let storage = VariableStorage::Boolean(
                            primitives::Bool::from(
                                literal.raw.parse::<bool>().unwrap()
                            )
                        );
                        // Return the storage
                        Some(storage)
                    }
                    _ => todo!()
                }
            },
            Expression::Assignment(assignment) => {
                // Evaluate the expression
                let value = self.evaluate(&assignment.expression);
                // Assign the value
                self.assign(assignment.identifier(), value.unwrap());
                // Return nothing
                None
            },
            Expression::Math(_) => todo!(),
            Expression::Group(_) => todo!(),
            Expression::Block(_) => todo!(),
            Expression::Program(program) => {
                // Iterate over the expressions
                for expression in &program.children {
                    // Evaluate the expression
                    self.evaluate(expression);
                }
                // Return nothing
                None
            },
            Expression::Return(_) => todo!(),
            Expression::Token(_) => todo!(),
            Expression::Conditional(_) => todo!(),
            Expression::Misc => todo!(),
        }
    }

    pub fn literal(&self, expression: &LiteralExpression) {
        use parser::types::ast::LiteralType;

        // match expression.which {
        //     LiteralType::Integer => {
        //         println!("{}", expression);
        //     },
        //     LiteralType::String => {
        //         println!("{}", expression.value);
        //     }
        //     LiteralType::Float => todo!(),
        //     LiteralType::Boolean => todo!(),
        // }
    }
}