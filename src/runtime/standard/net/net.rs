use crate::runtime::{Callable, Runtime, primitives::{VariableStorage, Scope, CompoundString}};
use reqwest::blocking::get;

pub struct NetGet;
impl Callable for NetGet {
    fn call(&self, _runtime: &mut Runtime, _parent: Scope, args: Vec<VariableStorage>) -> Option<VariableStorage> {
        // First param will be url. Error if it doesn't exist
        let url = args[0].clone();
        let url = match url {
            VariableStorage::String(i) => i,
            _ => panic!("Expected string")
        };

        let resp = get(&url.store).unwrap();
        let body = resp.text().unwrap();

        Some(VariableStorage::String(
            CompoundString::from(
                body
            )
        ))
    }

    fn name(&self) -> String {
        "net_get".to_string()
    }
}