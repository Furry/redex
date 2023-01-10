use super::token::{TokenType, Token};

pub enum LiteralKind { Integer, String, Boolean }
pub struct LiteralNode {
    pub value: String,
    pub kind: LiteralKind
}

pub enum MathOperation { Add, Subtract, Multiply, Divide, Modulo, Exponent }
pub struct MathNode {
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub operation: MathOperation
}

pub enum Node {
    Literal(LiteralNode),
    Math(MathNode)
}

pub struct Program {
    pub statements: Vec<Node>
}

impl Node {
    pub fn from<T: Into<Token>>(token: Token) -> Node {
        let token: Token = token.into();
        let kind = token.token_type.clone();

        if kind.is_math() {
            let operation = match kind {
                TokenType::Plus => MathOperation::Add,
                TokenType::Minus => MathOperation::Subtract,
                TokenType::Asterisk => MathOperation::Multiply,
                TokenType::Slash => MathOperation::Divide,
                _ => panic!("Invalid math operation")
            };

            Node::Math(MathNode {
                left: None,
                right: None,
                operation
            })
        } else if kind.is_literal() {
            let kind = match kind {
                TokenType::IntegerLiteral => LiteralKind::Integer,
                TokenType::StringLiteral => LiteralKind::String,
                TokenType::BooleanLiteral => LiteralKind::Boolean,
                _ => panic!("Invalid literal kind")
            };

            Node::Literal(LiteralNode {
                value: token.literal,
                kind
            })
        } else {
            panic!("Invalid node type");
        }
    }

    pub fn into_math(token: &Token) -> MathNode {
        let kind = token.token_type.clone();
        let operation = match kind {
            TokenType::Plus => MathOperation::Add,
            TokenType::Minus => MathOperation::Subtract,
            TokenType::Asterisk => MathOperation::Multiply,
            TokenType::Slash => MathOperation::Divide,
            _ => panic!("Invalid math operation")
        };

        return MathNode {
            left: None,
            right: None,
            operation
        }
    }
}