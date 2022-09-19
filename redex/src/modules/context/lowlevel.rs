use lexer::instance::collector::TokenGenerator;

pub struct LowLevel {
    pub lexer: lexer::instance::Instance
}

impl LowLevel {
    pub fn new() -> Self {
        Self { 
            lexer: 
                lexer::instance::Instance::new(),
        }
    }

    pub fn parse(&mut self, generator: TokenGenerator) -> () {
        parser::Parser::new(generator).parse();
    }
}