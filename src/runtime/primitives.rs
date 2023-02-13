mod integer;
mod string;
mod boolean;
mod function;
pub mod traits;

use std::collections::HashMap;

// Export integer::Integer
pub use integer::Integer;
pub use string::CompoundString;
pub use boolean::Bool;
pub use function::Function;

use self::traits::StdConversions;

use super::Callable;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    pub variables: HashMap<String, VariableStorage>
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            variables: HashMap::new()
        }
    }

    pub fn assign(&mut self, variable: Variable) {
        self.variables.insert(variable.name, *variable.value);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    Integer,
    String,
    Boolean,
    Function
}

trait DynFunction: Callable + Clone {}
#[derive(Debug, Clone, PartialEq)]
pub enum VariableStorage {
    Integer(Integer),
    String(CompoundString),
    Boolean(Bool),
    Function(Function),
    Scope(Scope)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub kind: VariableType,
    pub value: Box<VariableStorage>
}

impl StdConversions for VariableStorage {
    fn to_integer(&self) -> Integer {
        match self {
            VariableStorage::Integer(i) => i.clone(),
            VariableStorage::String(s) => s.to_integer(),
            VariableStorage::Boolean(b) => b.to_integer(),
            VariableStorage::Function(f) => f.to_integer(),
            VariableStorage::Scope(_) => panic!("Cannot convert scope to integer")
        }
    }

    fn to_compound_string(&self) -> CompoundString {
        match self {
            VariableStorage::Integer(i) => i.to_compound_string(),
            VariableStorage::String(s) => s.clone(),
            VariableStorage::Boolean(b) => b.to_compound_string(),
            VariableStorage::Function(f) => f.to_compound_string(),
            VariableStorage::Scope(_) => panic!("Cannot convert scope to string")
        }
    }

    fn to_bool(&self) -> Bool {
        match self {
            VariableStorage::Integer(i) => i.to_bool(),
            VariableStorage::String(s) => s.to_bool(),
            VariableStorage::Boolean(b) => b.clone(),
            VariableStorage::Function(f) => f.to_bool(),
            VariableStorage::Scope(_) => panic!("Cannot convert scope to boolean")
        }
    }
}