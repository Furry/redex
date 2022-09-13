use lexer::instance::collector::{Token, TokenTuple};

pub struct LowLevel {
    lexer: lexer::instance::Instance
}

impl LowLevel {
    pub fn new() -> Self {
        Self { 
            lexer: 
                lexer::instance::Instance::new()
        }
    }

    pub fn lex<S: Into<String>>(&mut self, input: S) -> Vec<TokenTuple> {
        let input: String = input.into();
        return self.lexer.tokenize(input);
    }
}