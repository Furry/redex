use super::Instance;

enum CollectorState {
    String,
    Numeric,
    Identifier,
    Whitespace,
    Comment,
    Decimal
}

pub enum Token {
    String(String),
    Numeric(String),
    Identifier(String),
    Whitespace(String),
    Comment(String),
    Decimal(String)
}

pub struct Collector {
    state: CollectorState
}

impl Collector {
    pub fn tokenize<S: Into<String>>(&mut self, instance: &mut Instance, input: S) -> Vec<Token> {
        let input: String = input.into();
        let cache: String = String::new();

        for (i, c) in input.chars().enumerate() {
        }

        todo!()
    }
}