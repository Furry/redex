use lexer::instance::collector::TokenGenerator;
use parser::modules::Branch;

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

    pub fn evaluate(mut branch: Branch) {
        println!("Evaluating expression: {:?}", branch);
        println!("{:?}", branch.take_right());
        println!("{:?}", branch);
    }
}