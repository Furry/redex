use std::collections::HashMap;

use super::{ traits::StdConversions, VariableStorage };

#[derive(Debug, Clone, PartialEq)]
pub struct Dict {
    pub store: HashMap<String, VariableStorage>
}

impl Dict {
    pub fn new() -> Dict {
        Dict {
            store: HashMap::new()
        }
    }

    pub fn from(value: HashMap<String, VariableStorage>) -> Dict {
        Dict {
            store: value
        }
    }

    pub fn from_string(value: String) -> Dict {
        let mut map = HashMap::new();
        let trimmed_value = value.trim_start_matches('[').trim_end_matches(']');
        let pairs: Vec<&str> = trimmed_value.split(',').collect();
        for pair in pairs {
            let key_value: Vec<&str> = pair.split(':').collect();
            if key_value.len() == 2 {
                let key = key_value[0].trim().trim_start_matches('"').trim_end_matches('"').to_string();
                let value = key_value[1].trim().to_string();
                map.insert(key, super::resolve(value));
            }
        }

        Dict {
            store: map
        }
    }
}

impl StdConversions for Dict {
    fn to_integer(&self) -> super::Integer {
        super::Integer::from(self.store.len() as f64)
    }

    fn to_compound_string(&self) -> super::CompoundString {
        super::CompoundString::from(format!("{:?}", self.store))
    }

    fn to_bool(&self) -> super::Bool {
        super::Bool::from(self.store.len() > 0)
    }
}
