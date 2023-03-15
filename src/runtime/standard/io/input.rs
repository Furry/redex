use crate::runtime::{Callable, primitives::{Scope, VariableStorage, Variable, traits::StdConversions, CompoundString}, Runtime};

pub struct ReadLn;
impl Callable for ReadLn {
    fn call(&self, _runtime: &Runtime, _parent: Scope, args: Vec<VariableStorage>) -> Option<VariableStorage> {

        // Hold the process until standard input is received.
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        Some(VariableStorage::String(
            CompoundString::from(input)
        ))
    }

    fn name(&self) -> String {
        "print".to_string()
    }
}