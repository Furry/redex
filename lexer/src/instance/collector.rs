use regex::Regex;
use strum::{EnumString, EnumIter, IntoStaticStr, Display};

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
        (Token::String, Regex::new(r#"^"[^"]+?""#).unwrap()),
        (Token::Comment, Regex::new(r"^//[^\n]+").unwrap()),
        (Token::Other, Regex::new(r"^[\s\S]").unwrap())
    ];
}

#[derive(Debug, PartialEq, Clone, Copy, Display, EnumIter)]
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
pub type TokenTuple = (Token, String, usize, usize);
pub struct Collector {
}

impl Collector {
    fn which(input: String, index: usize) -> TokenTuple {
        for (token, regex) in MATCHES.iter() {
            if regex.is_match(&input[index..]) {
                let captures = regex.captures(&input[index..]).unwrap();
                let start = captures.get(0).unwrap().start();
                // let end = captures.get(0).unwrap().end();
                let value = captures.get(0).unwrap().as_str().to_string();
                dbg!(start);
                return (*token, value.clone(), start, value.len());
            }
        }
        Err("No match found").unwrap()
    }

    pub fn tokenize(&self, input: String) -> Vec<TokenTuple> {
        let mut tokens = Vec::new();
        let mut index = 0;
        while index < input.len() {
            let (token, value, start, length) = Collector::which(input.clone(), index);
            index += start + length;
            tokens.push((
                token, value,
                index - 1, length
            ));
        }
        tokens
    }
}