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

    pub fn parse(&mut self, input: TokenGenerator) -> () {
        let mut ast = parser::modules::AST::new(input);
        ast.parse();
    }
}