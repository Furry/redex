use parser::types::ast::{Expression, LiteralExpression};

pub mod primitives;

pub struct Runtime {
    // pub variables: HashMap<String, primitives::Integer>,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            // variables: HashMap::new(),
        }
    }
}   

// Debug Functions
impl Runtime {
    pub fn coredump(&self) {
        // for (key, value) in &self.variables {
        //     println!("{}: {}", key, value.to_string());
        // }
    }
}

impl Runtime {
    pub fn evaluate(&self, expression: &Expression) {
        match expression {
            Expression::Literal(e) => todo!(),
            Expression::Assignment(_) => todo!(),
            Expression::Math(m) => {
                // Perform the operation
                let mut left: LiteralExpression;
                let mut right: LiteralExpression;

                match &m.right {
                    Some(r) => {
                        right = match &r {
                            // Expression::Literal(r) => r.clone(),
                            _ => todo!(),
                        }
                    },
                    None => todo!(),
                }
            },
            Expression::Group(_) => todo!(),
            Expression::Block(_) => todo!(),
            Expression::Program(p) => {
                // for child in p.children {
                //     self.evaluate(&child);
                // }
            },
            Expression::Token(_) => todo!(),
            Expression::Misc => todo!(),
            _ => todo!()
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