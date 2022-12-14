use regex::Regex;
use strum::{ EnumIter, Display };
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

lazy_static::lazy_static! {
    static ref MATCHES: [(Token, Regex); 13] = [
        (Token::EndOfLine, Regex::new(r"^(?:(?:\r?\n)|;)").unwrap()),
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
    EndOfLine,
    Other
}

pub type TokenTuple = (Token, String, usize, usize);
pub type TokenGenerator = Pin<Box<dyn Generator<Yield = TokenTuple, Return = ()>>>;

pub struct Collector;

impl Collector {
    /// Given an inputed string and a starting index, determine the next token type and it's length.
    /// # Arguments
    /// * `input` - The input string to tokenize.
    /// * `index` - The starting index of the token.
    /// # Examples
    /// ```
    /// use lexer::instance::collector::{ Token, TokenTuple, Collector };
    /// let mut collector = Collector {};
    /// let input = "2 + 2";
    /// let index = 0;
    /// let (token, length) = collector.next_token(input, index);
    /// assert_eq!(token, Token::Number);
    /// ```
    fn which(input: String, index: usize) -> TokenTuple {
        for (token, regex) in MATCHES.iter() {
            if regex.is_match(&input[index..]) {
                let captures = regex.captures(&input[index..]).unwrap();
                let value = captures.get(0).unwrap().as_str().to_string();
                return (*token, value.clone(), index, value.len());
            }
        }
        Err("No match found").unwrap()
    }

    /// Given an inputed string, tokenize it and return a generator that yields tokens until the end of the string.
    /// # Arguments
    /// * `input` - The input string to tokenize.
    /// # Examples
    /// ```
    /// use lexer::instance::collector::{ Token, TokenTuple, Collector };
    /// let mut collector = Collector {};
    /// let input = "2 + 2";
    /// let mut generator = collector.generator(input);
    /// let (token, value, index, length) = generator.as_mut().resume(());
    /// assert_eq!(token, Token::Number);
    /// ```
    pub(crate) fn tokenize<S: Into<String>>(input: S) -> TokenGenerator {
        let input: String = input.into();
        let mut index: usize = 0;
        return Box::pin(move || {
            loop {
                if index != input.len().try_into().unwrap() {
                    let token = Collector::which(input.clone(), index);
                    index += token.3;
                    yield token;
                } else {
                    return;
                }
            }
        });
    }

    pub fn vec(mut generator: TokenGenerator) -> Vec<TokenTuple> {
        let mut vec: Vec<TokenTuple> = Vec::new();
        loop {
            match Pin::new(&mut generator).resume(()) {
                GeneratorState::Yielded(value) => {
                    vec.push(value);
                },
                GeneratorState::Complete(_) => break
            }
        }

        vec
    }

}
