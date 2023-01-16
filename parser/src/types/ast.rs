use serde::{ Serialize, Deserialize };

use super::token::TokenType;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ExpressionMeta {
    pub start: usize,
    pub end: usize
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum LiteralType {
    String,
    Integer,
    Float,
    Boolean
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct LiteralExpression {
    pub meta: ExpressionMeta,
    pub raw: String,
    pub which: LiteralType
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum MathType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct MathExpression {
    pub meta: ExpressionMeta,
    pub left: Option<Box<Expression>>,
    pub right: Option<Box<Expression>>,
    pub which: MathType
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Program {
    pub meta: ExpressionMeta,
    pub children: Vec<Expression>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GroupExpression {
    pub meta: ExpressionMeta,
    pub children: Vec<Expression>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlockExpression {
    pub meta: ExpressionMeta,
    pub children: Vec<Expression>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ProgramBody {
    pub meta: ExpressionMeta,
    pub children: Vec<Expression>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Literal(LiteralExpression),
    Math(MathExpression),
    Group(GroupExpression),
    Block(BlockExpression),
    Program(ProgramBody),
    Misc
}

impl LiteralType {
    pub fn from(kind: TokenType) -> LiteralType {
        match kind {
            TokenType::StringLiteral => LiteralType::String,
            TokenType::IntegerLiteral => LiteralType::Integer,
            // TokenType::FloatLiteral => LiteralType::Float,
            TokenType::BooleanLiteral => LiteralType::Boolean,
            _ => panic!("Invalid literal type")
        }
    }
}

impl ExpressionMeta {
    pub fn new(start: usize, end: usize) -> ExpressionMeta {
        ExpressionMeta {
            start,
            end
        }
    }
}

impl MathType {
    pub fn from(kind: TokenType) -> MathType {
        match kind {
            TokenType::Plus => MathType::Add,
            TokenType::Minus => MathType::Subtract,
            TokenType::Asterisk => MathType::Multiply,
            TokenType::Slash => MathType::Divide,
            _ => panic!("Invalid math type")
        }
    }
}

impl Expression {
    pub fn meta(&self) -> ExpressionMeta {
        match self {
            Expression::Literal(literal) => literal.meta.clone(),
            Expression::Math(math) => math.meta.clone(),
            Expression::Group(group) => group.meta.clone(),
            Expression::Block(block) => block.meta.clone(),
            Expression::Program(program) => program.meta.clone(),
            Expression::Misc => ExpressionMeta::new(0, 0)
        }
    }
}