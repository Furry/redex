pub mod collector;

use self::collector::{Collector, TokenGenerator};

pub struct Instance {
    #[allow(dead_code)] // This will be used to track lexing jobs in the future. Maybe.
    handles: Vec<Box<TokenGenerator>>
}

impl Instance {
    pub fn new() -> Instance {
        Instance {
            handles: Vec::new()
        }
    }

    pub fn generator<S: Into<String>>(&mut self, input: S) -> TokenGenerator {
        return Collector::tokenize(input);
    }
}