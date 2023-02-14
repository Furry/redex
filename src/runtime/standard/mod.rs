pub mod io;

use crate::runtime::Callable;

// Re-export output
pub use self::io::output::Print;
pub use self::io::output::PrintLine;

use std::collections::HashMap;
use std::thread::Scope;

use super::primitives::VariableStorage;
lazy_static::lazy_static! {
    pub static ref IO: HashMap<String, dyn Fn(&Scope, Vec<VariableStorage>)> = {
        let mut map = HashMap::new();
        map.insert("print".to_string(), Box::new(Print));
        map.insert("println".to_string(), Box::new(PrintLine));
        map
    };
}