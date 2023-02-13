pub trait StdConversions {
    fn to_integer(&self) -> super::Integer;
    fn to_bool(&self) -> super::Bool;
    fn to_compound_string(&self) -> super::CompoundString;
}

pub trait Callable {
    fn call(&self, parent: super::Scope, args: Vec<super::VariableStorage>) -> Option<super::VariableStorage>;
    fn name(&self) -> String;
}