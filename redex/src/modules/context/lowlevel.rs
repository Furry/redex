pub struct LowLevel {
    pub lexer: lexer::instance::Instance
}

impl LowLevel {
    pub fn new() -> Self {
        Self { 
            lexer: 
                lexer::instance::Instance::new()
        }
    }
}