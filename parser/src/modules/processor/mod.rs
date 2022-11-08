pub enum ExpressionType {
}

pub struct Rule {
    pub name: String,
    pub expression: ExpressionType,
}

pub struct Processor {
    pub rules: Vec<Rule>,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            rules: Vec::new()
        }
    }
}