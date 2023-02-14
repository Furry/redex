pub mod io;

use crate::runtime::Callable;

// Re-export output
pub use self::io::output::Print;
pub use self::io::output::PrintLine;

use std::collections::HashMap;
lazy_static::lazy_static! {
    // Create a map of all the functions
    static ref HASHMAP: HashMap<u32, dyn Callable + Send + Sync> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}