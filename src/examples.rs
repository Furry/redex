use std::io::Write;

use parser::{ parser::Parser, lexer };
use crate::runtime::Runtime;

pub fn fizzbuzz() {
    super::execute(r#"
    let i = 0
    while (true) {
        let a = i % 3
        let b = i % 5
        print(i)
        if (a == 0) {
            print("Fizz")
        }
        if (b == 0) {
            print("Buzz")
        }
        println("")
        let i = i + 1
    }"#)
}

pub fn stdio() {
    super::execute(r#"
        println("Type any content and it will get echoed back to you.")
        while (true) {
            let input = readln()
            let out = "you entered: " + input
            println(out)
        }
    "#)
}

pub fn interpreter() {
    let mut runtime = Runtime::new();
    runtime.link_std();

    loop {
        print!("redex> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "exit" {
                    break;
                } else {
                    let ast = Parser::new(
                        lexer::parse_tokens(&input)
                    ).parse();

                    runtime.evaluate(&ast);
                }
            }
            Err(error) => println!("Error reading input: {}", error),
        }
    }
}