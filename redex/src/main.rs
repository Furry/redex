pub mod modules;
#[cfg(test)]
pub mod tests;

fn main() {
    let code = r#"
let x = 53 - (64 + 22) + 35221 * 635242;
let potato = "delicious~" // yum yum;
"#;
    // let code = "53 - (64 + 22) + 35221 * 635242 + \"hewwo~ uwu~\"";
    let mut ctx = modules::context::Context::new();
    let r = ctx.lowlevel.lex(code);
    println!("{:?}", r);
}
