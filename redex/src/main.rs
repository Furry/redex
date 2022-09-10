pub mod modules;

use lexer::instance::Instance;
use regex::Regex;

fn main() {
    let code = "3 + 4";
    let mut ctx = modules::context::Context::new();
    ctx.lowlevel.lex(code)
}
