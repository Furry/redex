use serde::{ Serialize, Deserialize };

use super::token::{TokenType, Token};

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
pub struct ReturnExpression {
    pub meta: ExpressionMeta,
    pub value: Box<Option<Expression>>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct LiteralExpression {
    pub meta: ExpressionMeta,
    pub raw: String,
    pub which: LiteralType
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct CallExpression {
    pub meta: ExpressionMeta,
    pub callee: Box<Expression>,
    pub args: Vec<Expression>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct IdentifierExpression {
    pub meta: ExpressionMeta,
    pub name: String
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
    pub scope: Vec<IdentifierExpression>,
    pub meta: ExpressionMeta,
    pub children: Vec<Expression>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct FunctionExpression {
    pub meta: ExpressionMeta,
    pub scope: Vec<IdentifierExpression>,
    pub children: Vec<Expression>,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct AssignmentExpression {
    pub meta: ExpressionMeta,
    pub identifier: Box<Expression>,
    pub expression: Box<Expression>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ConditionalExpression {
    pub meta: ExpressionMeta,
    pub condition: Box<Expression>,
    pub expression: Box<Expression>,
    pub alternative: Option<Box<Expression>>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Literal(LiteralExpression),
    Assignment(AssignmentExpression),
    Math(MathExpression),
    Group(GroupExpression),
    Block(BlockExpression),
    Program(ProgramBody),
    Return(ReturnExpression),
    Token(Token),
    Conditional(ConditionalExpression),
    Function(FunctionExpression),
    Call(CallExpression),
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
            Expression::Assignment(assignment) => assignment.meta.clone(),
            Expression::Return(return_expr) => return_expr.meta.clone(),
            Expression::Token(token) => ExpressionMeta::new(token.start, token.end),
            Expression::Conditional(conditional) => conditional.meta.clone(),
            Expression::Function(function) => function.meta.clone(),
            Expression::Call(call) => call.meta.clone(),
            Expression::Misc => ExpressionMeta::new(0, 1),
        }
    }

    pub fn is(&self, kind: TokenType) -> bool {
        match self {
            Expression::Token(token) => token.token_type == kind,
            _ => false
        }
    }
}

impl Into<String> for IdentifierExpression {
    fn into(self) -> String {
        return self.name
    }
}

impl AssignmentExpression {
    pub fn identifier(&self) -> String {
        match &*self.identifier {
            Expression::Token(token) => token.literal.clone(),
            _ => panic!("Invalid identifier")
        }
    }
}