pub mod collector;

use std::collections::HashMap;

pub struct Instance {
    pub rules: HashMap<String, Vec<String>>
}

impl Instance {
    pub fn new() -> Instance {
        Instance {
            rules: HashMap::new()
        }
    }
}