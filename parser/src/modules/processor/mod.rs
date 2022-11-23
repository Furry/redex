use std::{collections::VecDeque, ops::{GeneratorState, Generator}, pin::Pin};

use lexer::instance::collector::{TokenGenerator, Token, TokenTuple};

use super::ast::{Program, self, Expression, LiteralExpression, ExpressionMeta, MathExpression};

pub enum ExpressionType {
    Literal
}

pub struct Rule {
    pub name: String,
    pub expression: ExpressionType,
}

pub struct Processor {
    pub rules: Vec<Rule>,
    gen: TokenGenerator,
    pub input: String,
    position: usize,
    cache: Vec<TokenTuple>
}

impl Processor {
    pub fn new(input: &String) -> Self {
        Self {
            gen: lexer::instance::Instance::new().generator(input),
            input: input.to_string(),
            rules: Vec::new(), // ! Probably don't need this.
            position: 0,
            cache: Vec::new()
        }
    }
}

impl Processor {
    pub fn is_math(input: &String) -> bool {
        match input.as_str() {
            "+" | "-" | "*" | "/" | "%" | "^" => true,
            _ => false
        }
    }

    pub fn to_math(input: &String) -> ast::MathType {
        match input.as_str() {
            "+" => ast::MathType::Add,
            "-" => ast::MathType::Subtract,
            "*" => ast::MathType::Multiply,
            "/" => ast::MathType::Divide,
            "%" => ast::MathType::Modulo,
            _ => panic!("Invalid math type")
        }
    }

    // Start for generator traversal operations
    pub fn next(&mut self) -> Option<TokenTuple> {
        // If 'next' is an item in cache, take from cache. Else request from the generator.
        
        if self.position < self.cache.len() {
            let token = self.cache.get(self.position)
                .clone();

            self.position += 1;
            return token.cloned(); // To clone out of the borrow.
        } else {
            match Pin::new(&mut self.gen).resume(()) {
                GeneratorState::Yielded(token) => {
                    self.cache.push(token.clone());
                    self.position += 1;
                    return Some(token);
                },
                GeneratorState::Complete(_) => None
            }
        }
    }

    pub fn previous(&self) -> Option<Token> {
        if self.position == 0 {
            return None;
        } else {
            return Some(self.cache.get(self.position - 1).unwrap().clone().0);
        }
    }

    pub fn nth(&self, pos: usize) -> Option<Token> {
        if pos < self.cache.len() {
            return Some(self.cache.get(pos).unwrap().clone().0);
        } else {
            return None;
        }
    }

    pub fn next_expression(&mut self) -> Expression {
        let mut token_stack: VecDeque<Token> = VecDeque::new();
        let mut expression_stack: VecDeque<Expression> = VecDeque::new();
        let mut depth: usize = usize::try_from(0).unwrap();
        let mut discover: bool = false;


        loop {
            match self.next() {
                Some(token) => {
                    if token.0 == Token::Number {
                        return Expression::Literal(LiteralExpression {
                            meta: ExpressionMeta {
                                start: self.position,
                                end: token.3
                            },
                            raw: token.1,
                            which: ast::LiteralType::Integer
                        })
                    }
                }
                None => {
                    panic!("Unexpected end of input.");
                }
            }
        }
    }

    pub fn process(&mut self) {
        let mut program = Program {
            meta: ast::ExpressionMeta {
                start: 0,
                end: self.input.len()
            },
            children: Vec::new()
        };

        let mut token_stack: VecDeque<Token> = VecDeque::new();
        let mut expression_stack: VecDeque<Expression> = VecDeque::new();
        let mut depth: usize = usize::try_from(0).unwrap();
        let mut discover: bool = false;

        loop {
            match Pin::new(&mut self.gen).resume(()) {
                GeneratorState::Yielded(value) => {
                    self.position += 1;
                    // Destructure
                    let (token, literal, start, end) = value;
                    match token {
                        Token::Whitespace => {},
                        Token::String | Token::Number => {
                            expression_stack.push_front(Expression::Literal(
                                LiteralExpression {
                                    meta: ExpressionMeta {
                                        start: start,
                                        end: end
                                    },
                                    raw: literal,
                                    which: match token {
                                        Token::String => ast::LiteralType::String,
                                        Token::Number => ast::LiteralType::Integer,
                                        _ => panic!("Invalid literal type")
                                    }
                                }
                            ));
                        },
                        _ => {
                            token_stack.push_front(token);
                            if Self::is_math(&literal) {
                                expression_stack.push_back(self.next_expression());
                                let left = expression_stack.pop_front().unwrap();
                                let right = expression_stack.pop_front().unwrap();
                                expression_stack.push_front(
                                    Expression::Math(MathExpression {
                                        meta: ExpressionMeta {
                                            start: start,
                                            end: end
                                        },
                                        left: Box::new(left),
                                        right: Box::new(right),
                                        which: Self::to_math(&literal)
                                    })
                                )
                            }
                        }
                    }
                },
                GeneratorState::Complete(_) => break
            }
        }
        // println!("{:?}", stack)
    }
}