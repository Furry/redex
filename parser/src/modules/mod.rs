use lexer::instance::collector::TokenGenerator;

pub struct Branch;
pub struct AST {
    pub branches: Vec<Branch>,
    pub gen: TokenGenerator
}

impl AST {
    pub fn new(generator: TokenGenerator) -> Self {
        Self {
            branches: Vec::new(),
            gen: generator
        }
    }
}