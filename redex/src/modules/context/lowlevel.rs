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

    pub fn lex<S: Into<String>>(&mut self, input: S) {
        let input: String = input.into();
        println!("{:?}", self.lexer.tokenize(input));
    }
}