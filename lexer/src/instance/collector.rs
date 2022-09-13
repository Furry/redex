use regex::Regex;

lazy_static::lazy_static! {
    pub static ref MATCHES: [(Token, Regex); 12] = [
        (Token::Whitespace, Regex::new(r"^\s+").unwrap()),
        (Token::Identifier, Regex::new(r"^[a-zA-Z_]+").unwrap()),
        (Token::OpenParentheses, Regex::new(r"^\(").unwrap()),
        (Token::CloseParentheses, Regex::new(r"^\)").unwrap()),
        (Token::OpenBrace, Regex::new(r"^\{").unwrap()),
        (Token::CloseBrace, Regex::new(r"^\}").unwrap()),
        (Token::OpenBracket, Regex::new(r"^\[").unwrap()),
        (Token::CloseBracket, Regex::new(r"^\]").unwrap()),
        (Token::Number, Regex::new(r"^[0-9]+\.?[0-9]?+").unwrap()),
        (Token::String, Regex::new(r#""[^"]+?""#).unwrap()),
        (Token::Comment, Regex::new(r"//*+").unwrap()),
        (Token::Other, Regex::new(r"^[\s\S]").unwrap())
    ];
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Whitespace,
    Identifier,
    OpenParentheses,
    CloseParentheses,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Number,
    String,
    Comment,
    Other
}
    

pub struct Collector {
}

impl Collector {
    fn which(input: String, index: usize) -> (Token, String, usize, usize) {
        for (token, regex) in MATCHES.iter() {
            if regex.is_match(&input[index..]) {
                println!("MATCH!");
                let captures = regex.captures(&input[index..]).unwrap();
                let start = captures.get(0).unwrap().start();
                let end = captures.get(0).unwrap().end();
                let length = end - start;
                let value = captures.get(0).unwrap().as_str().to_string();
                return (*token, value, start, length);
            }
        }
        Err("No match found").unwrap()
    }

    pub fn tokenize(&self, input: String) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut index = 0;
        while index < input.len() {
            let (token, _, start, length) = Collector::which(input.clone(), index);
            tokens.push(token);
            index += start + length;
        }
        tokens
    }
}