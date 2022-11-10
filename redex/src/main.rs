#![feature(generator_trait)]
use std::{ops::{Generator, GeneratorState}, pin::Pin, collections::VecDeque};

use lexer::instance::collector::{Collector, Token};
use parser::modules::{ ast::{self, Expression, Program}, processor::Processor };
use modules::context::lowlevel::{self, LowLevel};

pub mod modules;
#[cfg(test)]
pub mod tests;

fn main() {

    let statement = "5 + (4 * 3)";
    let x = Expression::Math(ast::MathExpression {
        meta: ast::ExpressionMeta {
            start: 0,
            end: 11
        },

        left: Box::new(ast::Expression::Literal(ast::LiteralExpression {
            meta: ast::ExpressionMeta {
                start: 0,
                end: 1
            },

            raw: "5".to_string(),
            which: ast::LiteralType::Integer
        })),

        right: Box::new(ast::Expression::Math(ast::MathExpression {
            meta: ast::ExpressionMeta {
                start: 4,
                end: 11
            },

            left: Box::new(ast::Expression::Literal(ast::LiteralExpression {
                meta: ast::ExpressionMeta {
                    start: 4,
                    end: 5
                },

                raw: "4".to_string(),
                which: ast::LiteralType::Integer
            })),

            right: Box::new(ast::Expression::Literal(ast::LiteralExpression {
                meta: ast::ExpressionMeta {
                    start: 8,
                    end: 9
                },

                raw: "3".to_string(),
                which: ast::LiteralType::Integer
            })),
            which: ast::MathType::Multiply
        })),
        which: ast::MathType::Add
    });

    LowLevel::new().process(&"5 + (4 * 3)".to_string())
    // let mut gen = lowlevel::LowLevel::new().lexer.generator(statement);




}
