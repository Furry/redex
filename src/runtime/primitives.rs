mod integer;
mod string;
mod boolean;
mod function;
mod dict;
pub mod traits;

use std::collections::HashMap;

// Export integer::Integer
pub use integer::Integer;
pub use string::CompoundString;
pub use boolean::Bool;
pub use function::Function;
pub use dict::Dict;

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
    Function,
    Dict
}

trait DynFunction: Callable + Clone {}
#[derive(Debug, Clone)]
pub enum VariableStorage {
    Integer(Integer),
    String(CompoundString),
    Boolean(Bool),
    Dict(Dict),
    Function(Function),
    Scope(Scope)
}

impl PartialEq for VariableStorage {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Dict(l0), Self::Dict(r0)) => l0 == r0,
            (Self::Function(l0), Self::Function(r0)) => l0 == r0,
            (Self::Scope(l0), Self::Scope(r0)) => l0 == r0,
            _ => false,
        }
    }
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
            VariableStorage::Dict(_) => panic!("Cannot convert dict to integer"),
            VariableStorage::Scope(_) => panic!("Cannot convert scope to integer")
        }
    }

    fn to_compound_string(&self) -> CompoundString {
        match self {
            VariableStorage::Integer(i) => i.to_compound_string(),
            VariableStorage::String(s) => s.clone(),
            VariableStorage::Boolean(b) => b.to_compound_string(),
            VariableStorage::Function(f) => f.to_compound_string(),
            VariableStorage::Dict(d) => d.to_compound_string(),
            VariableStorage::Scope(_) => panic!("Cannot convert scope to string")
        }
    }

    fn to_bool(&self) -> Bool {
        match self {
            VariableStorage::Integer(i) => i.to_bool(),
            VariableStorage::String(s) => s.to_bool(),
            VariableStorage::Boolean(b) => b.clone(),
            VariableStorage::Function(f) => f.to_bool(),
            VariableStorage::Dict(d) => d.to_bool(),
            VariableStorage::Scope(_) => panic!("Cannot convert scope to boolean")
        }
    }
}

pub fn resolve(raw: String) -> VariableStorage {
    if raw.parse::<f64>().is_ok() { // Check if it's a number
        VariableStorage::Integer(Integer::from(raw.parse::<f64>().unwrap()))
    } else if raw.starts_with("\"") && raw.ends_with("\"") { // Check if it's a string
        VariableStorage::String(CompoundString::from(raw[1..raw.len() - 1].to_string()))
    } else if raw == "true" || raw == "false" { // Check if it's a boolean
        VariableStorage::Boolean(Bool::from(raw == "true"))
    } else if raw.starts_with("[") && raw.ends_with("]") { // Check if it's a dict
        VariableStorage::Dict(Dict::from_string(raw))
    } else {
        panic!("Unknown type: {}", raw);
    }
}