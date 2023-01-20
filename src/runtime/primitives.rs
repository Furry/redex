mod integer;
mod string;
mod traits;
mod boolean;

// Export integer::Integer
pub use integer::Integer;
pub use string::CompoundString;
pub use boolean::Bool;

#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    Integer,
    String,
    Boolean,
    Function
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableStorage {
    Integer(Integer),
    String(CompoundString),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub kind: VariableType,
    pub value: Box<VariableStorage>
}