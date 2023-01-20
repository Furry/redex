use std::{ collections::VecDeque };

use crate::types::{token::{Token, TokenType}, ast::{Expression, LiteralExpression, ExpressionMeta, LiteralType, GroupExpression, BlockExpression, MathExpression, MathType, ProgramBody, AssignmentExpression, IdentifierExpression, ReturnExpression}};

pub struct Parser {
    pub input: VecDeque<Token>,

    pos: usize, 
    len: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            len: tokens.len(),
            input: VecDeque::from(tokens),
            pos: 0
        }
    }
}

// Private functionality methods
impl Parser {
    fn scope(&mut self, tracked: TokenType) -> Vec<Token> {
        let mut depth = 1;
        let mut tokens = Vec::new();

        while let Some(token) = self.input.pop_front() {
            if token.token_type == tracked {
                depth += 1;
            }
            if token.token_type == tracked.get_closing().unwrap() {
                depth -= 1;
            }
            if depth == 0 {
                break;
            }
            tokens.push(token);
        }

        return tokens;
    }
}

impl Parser {
}

impl Parser {
    pub fn next(&mut self) -> Option<Expression> {
        loop {
            let next_ = self.input.pop_front();
            if next_.is_none() {
                return None;
            }
            let next = next_.unwrap();
            let kind = next.token_type.clone();
            let k = match kind {
                TokenType::StringLiteral  |
                TokenType::IntegerLiteral |
                TokenType::BooleanLiteral => {
                    Expression::Literal(
                        LiteralExpression {
                            meta: ExpressionMeta {
                                start: next.start,
                                end: next.end
                            },
                            raw: next.literal.clone(),
                            which: LiteralType::from(kind)
                        }
                    )
                },

                TokenType::Asterisk |
                TokenType::Slash |
                TokenType::Plus |
                TokenType::Minus => {
                    Expression::Math(
                        MathExpression {
                            meta: ExpressionMeta::new(next.start, next.end),
                            which: MathType::from(kind),
                            left: None,
                            right: None
                        }
                    )
                },

                TokenType::LBrace => {
                    let scope = self.scope(TokenType::LBrace);
                    let mut parser = Parser::new(scope);
                    let mut exprs = Vec::new();
                    while let Some(expr) = parser.next() {
                        exprs.push(expr);
                    }
                    Expression::Group(GroupExpression {
                        meta: ExpressionMeta::new(next.start, next.end),
                        children: exprs
                    })
                },

                TokenType::LParen => {
                    let scope = self.scope(TokenType::LParen);
                    let mut parser = Parser::new(scope);
                    let mut exprs = Vec::new();
                    while let Some(expr) = parser.next() {
                        exprs.push(expr);
                    }
                    Parser::parse_vec(exprs)
                }

                TokenType::Let => {
                    let identifier_ = self.next();
                    if identifier_.is_none() {
                        panic!("Expected identifier after let");
                    }
                    let identifier = identifier_.unwrap();

                    self.next(); // Skip the equals sign

                    let exp_ = self.next();
                    if exp_.is_none() {
                        panic!("Expected expression after identifier");
                    }
                    let exp = exp_.unwrap();

                    Expression::Assignment(
                        AssignmentExpression {
                            meta: ExpressionMeta::new(identifier.meta().start, exp.meta().end),
                            identifier: Box::new(identifier),
                            expression: Box::new(exp)
                        }
                    )
                }

                TokenType::Return => {
                    let exp = self.next();
                    let mut end = next.end;

                    if let Some(exp) = exp.clone() {
                        end = exp.meta().end;
                    }

                    Expression::Return(ReturnExpression {
                        meta: ExpressionMeta::new(next.start, end),
                        value: Box::new(exp)
                    })
                }

                TokenType::Fn => {
                    let identifier_ = self.next();
                    if identifier_.is_none() {
                        panic!("Expected identifier after fn");
                    }
                    let identifier = identifier_.unwrap();

                    let args_ = self.next();
                    if args_.is_none() {
                        panic!("Expected arguments after identifier");
                    }
                    let args = args_.unwrap();

                    let scope = self.scope(TokenType::LBrace);

                    let mut parser = Parser::new(scope);
                    let mut exprs = Vec::new();
                    while let Some(expr) = parser.next() {
                        exprs.push(expr);
                    }
                    // let body = Parser::parse_vec(exprs);

                    let mut scope_arguments: Vec<IdentifierExpression> = Vec::new();
                    match args.clone() {
                        Expression::Group(group) => {
                            for expr in group.children {
                                match expr {
                                    Expression::Token(token) => {
                                        scope_arguments.push(
                                            IdentifierExpression {
                                                meta: ExpressionMeta::new(token.start, token.end),
                                                name: token.literal
                                            }
                                        );
                                    },
                                    _ => panic!("Expected token for arguments"),
                                }
                            }
                        },
                        _ => panic!("Expected group expression for arguments"),
                    };

                    Expression::Assignment(
                        AssignmentExpression {
                            meta: ExpressionMeta::new(0, 0),
                            identifier: Box::new(identifier),
                            expression: Box::new(
                                Expression::Program(
                                    ProgramBody {
                                        meta: ExpressionMeta::new(0, 0),
                                        scope: scope_arguments,
                                        children: exprs
                                    }
                                )
                            )
                        }
                    )   
                }
                
                TokenType::Space => continue,
                _ => Expression::Token(next),
            };

            if k != Expression::Misc {
                return Some(k);
            }
        }
    }

    pub fn parse_vec(input: Vec<Expression>) -> Expression {
        let mut hold: Option<Expression> = None;
        let mut exprs: Vec<Expression> = Vec::new();

        if input.len() == 0 {
            return Expression::Group(
                GroupExpression {
                    meta: ExpressionMeta::new(0, 0),
                    children: Vec::new()
                }
            )
        }

        let start = input.get(0)
            .unwrap()
            .meta()
            .start;

        let end = input.get(input.len() - 1)
            .unwrap()
            .meta()
            .end;

        let mut input = input.into_iter();

        // Get the start of the first and the end of the last

        loop {
            let expr = input.next();
            if expr.is_none() {
                // If hold exists, push it
                if hold.is_some() {
                    exprs.push(hold.unwrap());
                }
                if exprs.len() == 1 {
                    return exprs.pop().unwrap();
                } else {
                    return Expression::Block(BlockExpression {
                        meta: ExpressionMeta::new(start, end),
                        children: exprs
                    });
                }
            }

            let expr = expr.unwrap();
            match expr.clone() {
                Expression::Math(mut math) => {
                    if hold.is_none() {
                        // panic!("Expected expression before math operator");
                        hold = Some(expr.clone());
                        continue;
                    }
                    math.left = Some(Box::new(hold.unwrap()));
                    math.right = Some(Box::new(input.next().unwrap()));
                    exprs.push(Expression::Math(math));
                    hold = None;
                },
                _ => {
                    hold = Some(expr);
                }
            }
        }
    }

    pub fn parse_all(&mut self) -> Expression {
        let mut hold: Option<Expression> = None;
        let mut exprs = Vec::new();

        loop {
            let expr = self.next();
            if expr.is_none() {
                // If hold is some, push it
                if hold.is_some() {
                    exprs.push(hold.unwrap());
                }
                if exprs.len() == 1 {
                    return exprs.pop().unwrap();
                } else {
                    return Expression::Block(BlockExpression {
                        meta: ExpressionMeta::new(self.pos, self.len),
                        children: exprs
                    });
                }
            }

            let expr = expr.unwrap();
            match expr.clone() {
                Expression::Math(mut math) => {
                    if hold.is_none() {
                        // panic!("Expected expression before math operator");
                        hold = Some(expr.clone());
                        continue;
                    }
                    math.left = Some(Box::new(hold.unwrap()));
                    math.right = Some(Box::new(self.next().unwrap()));
                    // exprspush(Expression::Math(math.clone()));
                    hold = Some(Expression::Math(math));
                    // exprs.push(Expression::Math(math));
                },
                _ => {
                    if hold.is_some() {
                        exprs.push(hold.unwrap());
                    }
                    hold = Some(expr);
                }
            };
        }
    }

    pub fn parse(&mut self) -> Expression {
        return Expression::Program(
            ProgramBody {
                scope: Vec::new(),
                meta: ExpressionMeta::new(0, self.len),
                children: vec![self.parse_all()]
            }
        )
    }
}