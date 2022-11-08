use super::processor::Processor;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpressionMeta {
    pub start: usize,
    pub end: usize
}

///////////////
#[derive(Serialize, Deserialize, Debug)]
pub enum LiteralType {
    String,
    Integer,
    Float,
    Boolean
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpressionLiteral {
    pub meta: ExpressionMeta,
    pub raw: String,
    pub which: LiteralType
}
//////////////

#[derive(Serialize, Deserialize, Debug)]
pub enum MathType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MathExpression {
    pub meta: ExpressionMeta,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub which: MathType
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Expression {
    Literal(ExpressionLiteral),
    Math(MathExpression)
}

pub struct AST {
    pub processor: Processor
}

impl AST {
    pub fn new() -> Self {
        Self {
            processor: Processor::new()
        }
    }
}

