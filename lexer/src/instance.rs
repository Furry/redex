pub mod collector;

use self::collector::{Collector, Token, TokenTuple};

pub struct Instance {
    collector: Collector
}

impl Instance {
    pub fn new() -> Instance {
        Instance {
            collector: Collector {}
        }
    }

    pub fn tokenize(&self, input: String) -> Vec<TokenTuple> {
        self.collector.tokenize(input)
    }
}