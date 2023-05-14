use std::collections::HashMap;

use crate::runtime::{Callable, Runtime, primitives::{VariableStorage, Scope, CompoundString, Integer}};
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
        let status = &resp.status().as_u16();
        let body = resp.text().unwrap();

        let mut temp: HashMap<String, VariableStorage> = HashMap::new();
        temp.insert("status".to_string(), VariableStorage::Integer(Integer::from(*status)));
        temp.insert("body".to_string(), VariableStorage::String(CompoundString::from(body)));

        Some(VariableStorage::Dict(
            crate::runtime::primitives::Dict {
                store: temp
            }
        ))
    }

    fn name(&self) -> String {
        "net_get".to_string()
    }
}