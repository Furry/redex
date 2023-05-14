use strum::{IntoEnumIterator, EnumProperty};

use crate::types::token::{TokenType, Token};

pub fn parse_tokens(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut input = input.clone();
    let mut pos: usize = 0;

    // While there's input left, keep looping over the Token enum.
    // If the regex matches, add the token to the tokens vector.
    // If the regex doesn't match, return an error.
    while input.len() > 0 {
        let mut matched = false;
        for token_type in TokenType::iter() {
            let regex = token_type.get_str("regex").unwrap();
            let re = regex::Regex::new(regex).unwrap();
            if let Some(captures) = re.captures(input) {
                let literal = captures.get(0).unwrap().as_str();
                let token = Token {
                    token_type: token_type.clone(),
                    literal: literal.to_string(),
                    start: pos,
                    end: pos + literal.len(),
                    size: literal.len(),
                };
                tokens.push(token);
                input = &input[literal.len()..];
                pos += literal.len();
                matched = true;
                break;
            }
        }
        if !matched {
            panic!("No token matched the input: {}", input);
        }
    }

    tokens
}
