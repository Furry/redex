use std::collections::VecDeque;

use types::{token::{Token, TokenType}, ast::Node};

extern crate strum;
extern crate strum_macros;

pub mod parser;
pub mod lexer;
pub mod types;