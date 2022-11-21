pub mod collector;

use std::ops::GeneratorState;
use std::ops::Generator;
use std::pin::Pin;

use self::collector::{Collector, TokenGenerator, TokenTuple};

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

    // pub fn iter(generator: TokenGenerator) -> core::slice::Iter<'static, TokenTuple> {
    //     let mut tokens: Vec<TokenTuple> = Vec::new();

    //     loop {
    //         match Pin::new(&mut generator.clone()).resume(()) {
    //             GeneratorState::Yielded(value) => {
    //                 tokens.push(value);
    //             },
    //             GeneratorState::Complete(_) => break
    //         }
    //     }

    //     return tokens.iter();
    // }
}