use std::{collections::HashMap, borrow::{BorrowMut, Borrow}};
use std::time::Instant;

use parser::types::ast::{Expression, LiteralExpression, MathType, IdentifierExpression};
use heapsize::HeapSizeOf;

use self::primitives::{ VariableStorage, Scope, CompoundString, Function };
use self::primitives::traits::StdConversions;

pub mod primitives;
pub mod standard;
pub mod errors;

// Re-export traits
pub use self::primitives::traits::{ Callable };

pub struct Runtime {
    pub global: Scope,
    pub instructions: u64,
    pub start_time: std::time::Instant,
    pub standard_functions: HashMap<u32, Box<dyn Callable>>
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            global: Scope::new(),
            instructions: 0,
            start_time: std::time::Instant::now(),
            standard_functions: HashMap::new()
        }
    }

    pub fn link_std(&mut self) {
        let mut count = 0;
        // Link our standard functions
        // Get the point of standard::Print::call
        self.assign("print".to_string(), VariableStorage::Function(
            Function::new(
                "print".to_string(),
                vec!["value".to_string()],
                vec![],
                Some(count)
            )
        ));
        self.standard_functions.insert(count, Box::new(standard::Print));
        count = count + 1;

        self.assign("println".to_string(), VariableStorage::Function(
            Function::new(
                "println".to_string(),
                vec!["value".to_string()],
                vec![],
                Some(count)
            )
        ));
        self.standard_functions.insert(count, Box::new(standard::PrintLine));
        count = count + 1;

        self.assign("readln".to_string(), VariableStorage::Function(
            Function::new(
                "readln".to_string(),
                Vec::new(),
                Vec::new(),
                Some(count)
            )
        ));

        self.standard_functions.insert(count, Box::new(standard::ReadLn));
        count = count + 1;
    }
}   

// Debug Functions
impl Runtime {
    pub fn coredump(&self) {
        // Get the size in bytes of the entire Runtime struct
        let mut size = self.heap_size_of_children();
        size = size + std::mem::size_of::<Runtime>();

        println!("========= CORE DUMP =========");
        println!("Runtime Size: {} bytes", size);
        println!("Instructions: {}", self.instructions);
        println!("Runtime: {}", self.start_time.elapsed().as_millis());
        println!("--------- SCOPES: ---------");
        println!("Global Scope:");
        for (key, value) in self.global.variables.iter() {
            println!("{}: {:?}", key, value);
        }
    }

    fn assign(&mut self, key: String, value: VariableStorage) {
        self.global.variables.insert(key, value);
    }

    fn pull(&self, key: &String) -> VariableStorage {
        let value = self.global.variables.get(key).unwrap();
        let value = value.clone();
        value
    }
}

impl Runtime {
    pub fn evaluate(&mut self, expression: &Expression) -> Option<VariableStorage> {
        // Increment the instruction counter
        self.instructions = self.instructions + 1;
        // println!("Expression: {:?}", expression);

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
                                literal.raw.parse::<f64>().unwrap()
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
            Expression::Math(math) => {
                // println!("Math: {:?}", math);
                // println!("hit");
                // Evaluate the left expression
                let left = self.evaluate(&math.left.clone().unwrap());
                // Evaluate the right expression
                let right = self.evaluate(&math.right.clone().unwrap());
                // Create a variable storage
                let storage = match math.which {
                    MathType::Add => {
                        // Add the two values
                        let left = left.unwrap();
                        let right = right.unwrap();
                        match left {
                            VariableStorage::Integer(left) => {
                                VariableStorage::Integer(left.add(&right.to_integer()))
                            },
                            VariableStorage::String(left) => {
                                VariableStorage::String(CompoundString::from(left.store + &right.to_compound_string().store))
                            },
                            _ => todo!()
                        }
                    },
                    MathType::Subtract => {
                        // Subtract the two values
                        let left = left.unwrap();
                        let right = right.unwrap();
                        match left {
                            VariableStorage::Integer(left) => {
                                VariableStorage::Integer(left.sub(&right.to_integer()))
                            },
                            _ => todo!()
                        }
                    },
                    MathType::Multiply => {
                        // Multiply the two values
                        let left = left.unwrap();
                        let right = right.unwrap();
                        match left {
                            VariableStorage::Integer(left) => {
                                VariableStorage::Integer(left.mult(&right.to_integer()))
                            },
                            _ => todo!()
                        }
                    },
                    MathType::Divide => {
                        // Divide the two values
                        let left = left.unwrap();
                        let right = right.unwrap();
                        match left {
                            VariableStorage::Integer(left) => {
                                VariableStorage::Integer(left.div(&right.to_integer()))
                            },
                            _ => todo!()
                        }
                    },
                    MathType::Modulo => {
                        // Modulus the two values
                        let left = left.unwrap();
                        let right = right.unwrap();
                        match left {
                            VariableStorage::Integer(left) => {
                                VariableStorage::Integer(left.modulo(&right.to_integer()))
                            },
                            _ => todo!()
                        }
                    },
                    MathType::Power => {
                        // Exponent the two values
                        let left = left.unwrap();
                        let right = right.unwrap();
                        match left {
                            VariableStorage::Integer(left) => {
                                VariableStorage::Integer(left.pow(&right.to_integer()))
                            },
                            _ => todo!()
                        }
                    }
                };
                // Return the storage
                Some(storage)
            },
            Expression::Group(group) => {
                None
            },
            Expression::Block(block) => {
                for expression in &block.children {
                    self.evaluate(expression);
                }
                None
            },
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
            Expression::Token(t) => {
                if self.global.variables.contains_key(t.literal.as_str()) {
                    Some(self.pull(&t.literal))
                } else {
                    None
                }
            },
            Expression::Conditional(_) => todo!(),
            Expression::Function(function) => {
                let names = function.scope.iter()
                    .map(|x| x.name.clone())
                    .collect::<Vec<String>>();

                Some(VariableStorage::Function(
                    primitives::Function::new(
                        function.name.clone(),
                        names,
                        function.children.clone(),
                        None
                    )
                ))
            }
            Expression::Misc => todo!(),
            Expression::Call(call) => {
                // Get the function
                let function = self.pull(&call.callee);

                // dbg!(call.args.clone());
                // Get the arguments
                let arguments: Vec<VariableStorage> = call.args.iter()
                    .map(|x| self.evaluate(x).unwrap())
                    .collect();

                // Match its type
                match function {
                    VariableStorage::Function(callablefunction) => {
                        let scope = Scope::new();
                        callablefunction.call(&self, scope, arguments)
                    },
                    _ => {
                        panic!("Cannot call a non-function");
                    }
                }
                // Return the result
            }
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