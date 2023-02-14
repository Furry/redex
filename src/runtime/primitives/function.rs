use parser::types::ast::Expression;

use crate::runtime::Callable;

use super::traits::StdConversions;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    name: String,
    parameters: Vec<String>,
    children: Vec<Expression>,
    // Store an optional pointer, which will point directly to a function.
    ptr: Option<u32>
}

impl Function {
    pub fn new(name: String, parameters: Vec<String>, children: Vec<Expression>, ptr: Option<u32>) -> Function {
        Function {
            name,
            parameters,
            children,
            ptr
        }
    }
}

impl StdConversions for Function {
    fn to_integer(&self) -> super::Integer {
        let ptr = self as *const Function as *const super::Integer;
        unsafe { ptr.read() }
    }

    fn to_compound_string(&self) -> super::CompoundString {
        super::CompoundString::from(
            format!("Function: {} <{}>", self.name, self.to_integer().store)
        )
    }

    fn to_bool(&self) -> super::Bool {
        super::Bool::from(true)
    }
}


impl Callable for Function {
    fn call(&self, parent: super::Scope, args: Vec<super::VariableStorage>) -> Option<super::VariableStorage> {

        // If self.ptr is Some, call the function unsafely via the pointer.
        if let Some(ptr) = self.ptr {
            let ptr = ptr as *const Function;
            let func = unsafe { ptr.read() };
            return func.call(parent, args);
        } else {
            // Create a new scope
            let mut scope = super::Scope::new();
    
            
            for (i, arg) in args.iter().enumerate() {
                // Create a variable
                let variable = super::Variable {
                    name: self.parameters[i].clone(),
                    kind: super::VariableType::Integer,
                    value: Box::new(arg.clone())
                };
                // Assign the variable
                scope.assign(variable);
            }
            // Create a variable storage
            let storage = super::VariableStorage::Scope(scope);
            // Return the storage
            Some(storage)
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}