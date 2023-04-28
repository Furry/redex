use std::{ collections::VecDeque };

use crate::types::{token::{Token, TokenType}, ast::{Expression, LiteralExpression, ExpressionMeta, LiteralType, GroupExpression, BlockExpression, MathExpression, MathType, ProgramBody, AssignmentExpression, IdentifierExpression, ReturnExpression, ConditionalExpression, FunctionExpression, CallExpression}};

#[derive(Debug)]
pub struct Parser {
    pub input: VecDeque<Token>,
    pub skip_call: bool,
    pos: usize, 
    len: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            len: tokens.len(),
            skip_call: false,
            input: VecDeque::from(tokens),
            pos: 0
        }
    }
}

// Private functionality methods
impl Parser {
    // A utility function to collect tokens until a corresponding token is found, balancing successive occurances.
    fn scope(&mut self, tracked: TokenType) -> Vec<Token> {
        let mut depth = 1;
        let mut count = 0;
        let mut tokens = Vec::new();

        while let Some(token) = self.input.pop_front() {
            // if count == 0 && token.token_type == tracked {
            //         depth -= 1;
            //     }
            // println!("TRACKED: {:?}; SEE: {:?} -- {}", tracked.get_closing().unwrap(), token.token_type, depth);
            if token.token_type == tracked {
                depth += 1;
            }
            if token.token_type == tracked.get_closing().unwrap() {
                depth -= 1;
            }
            if depth == 0 {
                break;
            }
            count += 1;
            tokens.push(token);
        }

        return tokens;
    }

    fn until_eol(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(token) = self.input.pop_front() {
            match token.token_type {
                TokenType::Semicolon |
                TokenType::Newline => {
                    break;
                }
                _ => {
                    tokens.push(token);
                }
            }
        }

        return tokens;
    }
}

impl Parser {
    pub fn peek_next(&mut self) -> Option<Expression> {
        // Copy self and perform a next() operation on the copy.
        let mut copy = Parser::new(self.input.clone().into());
        copy.next()
    }

    pub fn next(&mut self) -> Option<Expression> {
        // println!("Parsing next expression. {:?}", self.input);
        loop {
            // Base case: If there's no more input, return None.
            let next_ = self.input.pop_front();
            if next_.is_none() {
                return None;
            }
            let next = next_.unwrap();
        
            let kind = next.token_type.clone();

            // Match the type of token, and construct the appropriate expression.
            let k = match kind {

                // If it's a literal type, parse it into a literal expression.
                TokenType::StringLiteral  |
                TokenType::IntegerLiteral |
                TokenType::BooleanLiteral => {
                    let literal_content = next.literal.clone();
                    // If 'which' is a string literal, remove the quotes.
                    let literal_content = if kind == TokenType::StringLiteral {
                        literal_content[1..literal_content.len() - 1].to_string()
                    } else {
                        literal_content
                    };

                    Expression::Literal(
                        LiteralExpression {
                            meta: ExpressionMeta {
                                start: next.start,
                                end: next.end
                            },
                            raw: literal_content,
                            which: LiteralType::from(kind)
                        }
                    )
                },

                // Consruct a new math expression with the operator as the type.
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

                // Construct a scope by finding the closing brace, parse it into an array of expressions, and construct a group from it.
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

                // Construct a scope by finding the closing parenthesis, then parse the scope.
                TokenType::LParen => {
                    let scope = self.scope(TokenType::LParen);
                    let mut parser = Parser::new(scope);
                    let mut exprs = Vec::new();
                    while let Some(expr) = parser.next() {
                        exprs.push(expr);
                    }

                    // Drain all commas from the expression list.
                    exprs.retain(|expr| {
                        if let Expression::Token(token) = expr {
                            if token.token_type == TokenType::Comma {
                                return false;
                            }
                        }
                        true
                    });

                    // Check if there's only Identifiers
                    let mut only_identifiers = true;
                    for expr in exprs.clone() {
                        if let Expression::Token(t) = expr {
                            if t.token_type != TokenType::Identifier {
                                only_identifiers = false;
                            }
                        }
                    }

                    // If there's only identifiers, this is a grouping.
                    if only_identifiers {
                        Expression::Group(GroupExpression {
                            meta: ExpressionMeta::new(next.start, next.end),
                            children: exprs
                        })
                    } else {
                        Parser::parse_vec(exprs)
                    }
                },

                // Let:
                // Identifier -> Equals (Skip) -> Expression
                TokenType::Let => {
                    let identifier_ = self.next();
                    if identifier_.is_none() {
                        panic!("Expected identifier after let");
                    }
                    let identifier = identifier_.unwrap();

                    self.next(); // Skip the equals sign

                    
                    // Create new parser instance 

                    // let mut parser = Parser::new(self.input.clone().into());
                    // let exp = parser.parse_all();

                    let chars = self.until_eol();
                    let mut parser = Parser::new(chars);
                    let exp = parser.parse_all();

                    // let exp_ = self.next();
                    // if exp_.is_none() {
                    //     panic!("Expected expression after identifier");
                    // }
                    // let exp = exp_.unwrap();

                    Expression::Assignment(
                        AssignmentExpression {
                            meta: ExpressionMeta::new(identifier.meta().start, exp.meta().end),
                            identifier: Box::new(identifier),
                            expression: Box::new(exp)
                        }
                    )
                },

                // Return:
                // Identifer -> Expression (Optional)
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
                },

                // Fn (Function):
                // Identifier -> Arguments (Group) -> Scope
                // Basically, the scope is parsed into a program and then that program is assigned to an identifier using the AssignExpression
                TokenType::Fn => {
                    self.skip_call = true;
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

                    // dbg!(&scope);

                    let mut parser = Parser::new(scope);
                    let mut exprs = Vec::new();
                    while let Some(expr) = parser.next() {
                        exprs.push(expr);
                    }

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
                                    _ => {
                                        println!("{:?}", expr);
                                        // panic!("Expected token for argument");
                                    }
                                }
                            }
                        },
                        _ => panic!("Expected group expression for arguments"),
                    };

                    Expression::Assignment(
                        AssignmentExpression {
                            meta: ExpressionMeta::new(0, 0),
                            identifier: Box::new(identifier.clone()),
                            expression: Box::new(
                                Expression::Function(
                                    FunctionExpression {
                                        meta: ExpressionMeta::new(0, 0),
                                        scope: scope_arguments,
                                        children: exprs,
                                        name: match identifier {
                                            Expression::Token(token) => token.literal,
                                            _ => {
                                                println!("{:?}", identifier);
                                                panic!("Expected token for function name");
                                            }
                                        }
                                    }
                                )
                            )
                        }
                    )   
                },

                TokenType::Else => {
                    // We're expecting a block after this!
                    // self.next()
                    //     .unwrap_or_else(|| panic!("Expected block after else"));
                    Expression::Block(
                        BlockExpression {
                            meta: ExpressionMeta::new(next.start, next.end),
                            children: vec![
                                self.next()
                                    .unwrap_or_else(|| panic!("Expected block after else"))
                            ]
                        }
                    )
                }

                TokenType::If => {
                    let condition = self.next()
                        .unwrap_or_else(|| panic!("Expected condition after if"));

                    let expression = self.next()
                        .unwrap_or_else(|| panic!("Expected expression after if"));


                    let mut alternative: Option<Box<Expression>> = None;
                    if let Some(peek) = self.peek_next() {
                        match peek {
                            Expression::Token(token) => {
                                println!("Checking against: {:?}", token.literal);
                                if token.token_type == TokenType::Else {
                                    self.next();
                                    // alternative = Some(Box::new(self.next().unwrap_or_else(|| panic!("Expected expression after else"))));
                                    
                                }
                            }
                            _ => {}
                        }
                    }

                    Expression::Conditional(
                        ConditionalExpression {
                            meta: ExpressionMeta::new(next.start, next.end),
                            condition: Box::new(condition),
                            expression: Box::new(expression),
                            alternative
                        }
                    )
                },

                TokenType::Space |
                TokenType::Newline => continue,

                _ => {
                    let pn = self.peek_next();
                    // If pn is some and its a group, then this is a function call.
                    if self.skip_call {
                        // println!("Call Skipped");
                        self.skip_call = false;
                        self.next();
                        Expression::Token(next)
                    } else {
                        if let Some(pn) = pn {
                            if let Expression::Group(group) = pn {
                                let mut args: Vec<Expression> = group.children.clone();
                                let mut end = next.end;
    
                                // Consume the group
                                self.next();
    
                                // println!("Group: {:?}", group);
                                break Some(Expression::Call(
                                    CallExpression {
                                        meta: ExpressionMeta::new(next.start, end),
                                        callee: next.literal,
                                        args,
                                    }
                                ))
                            }
                        }
    
                        Expression::Token(next)
                    }
                }

            }; // END OF STATEMENT

            if k != Expression::Misc {
                // println!("Yielding {:?}", k.clone());
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
                        hold = Some(expr.clone());
                        continue;
                    }
                    math.left = Some(Box::new(hold.unwrap()));
                    math.right = Some(Box::new(self.next().unwrap()));
                    hold = Some(Expression::Math(math));
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