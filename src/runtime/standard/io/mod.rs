use crate::runtime::{Callable, Runtime, primitives::{VariableStorage, Scope}};

pub mod output;
pub mod input;

pub struct Sleep;
impl Callable for Sleep {
    fn call(&self, _runtime: &mut Runtime, _parent: Scope, args: Vec<VariableStorage>) -> Option<VariableStorage> {
        let time = args[0].clone();
        let time = match time {
            VariableStorage::Integer(i) => i,
            _ => panic!("Expected integer")
        };
        std::thread::sleep(std::time::Duration::from_millis(time.store as u64));
        None
    }

    fn name(&self) -> String {
        "sleep".to_string()
    }
}

pub struct Exit;
impl Callable for Exit {
    fn call(&self, _runtime: &mut Runtime, _parent: Scope, _args: Vec<VariableStorage>) -> Option<VariableStorage> {
        std::process::exit(0);
    }

    fn name(&self) -> String {
        "exit".to_string()
    }
}