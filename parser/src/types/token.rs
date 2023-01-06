use strum::{IntoEnumIterator, EnumProperty};
use strum_macros::{EnumIter, EnumProperty};
use regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub start: usize,
    pub end: usize,
    pub size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumProperty, EnumIter)]
pub enum TokenType {
    // Keywords
    #[strum(props(regex = "^let"))]
    Let,
    #[strum(props(regex = "^fn"))]
    Fn,
    // Identifiers
    // This regex for ' and " /.*?'((?:\\\\|\\'|[^'])*+)'/"
    #[strum(props(regex = r#"(?:^'((?:\\\\|\\'|[^'])*)')|(?:^"((?:\\\\|\\"|[^"])*)")"#))]
    StringLiteral,
    // Literals
    #[strum(props(regex = r#"^\d+(?:.\d+)?"#))]
    IntegerLiteral,
    #[strum(props(regex = r#"^(true|false)"#))]
    BooleanLiteral,
    // Operators
    #[strum(props(regex = r#"^="#))]
    Assign,
    #[strum(props(regex = r#"^\+"#))]
    Plus,
    #[strum(props(regex = r#"^-"#))]
    Minus,
    #[strum(props(regex = r#"^!"#))]
    Bang,
    #[strum(props(regex = r#"^\*"#))]
    Asterisk,
    #[strum(props(regex = r#"^/"#))]
    Slash,
    // Delimiters
    #[strum(props(regex = r#"^,"#))]
    Comma,
    #[strum(props(regex = r#"^;"#))]
    Semicolon,
    #[strum(props(regex = r#"^\("#))]
    LParen,
    #[strum(props(regex = r#"^\)"#))]
    RParen,
    #[strum(props(regex = r#"^\{"#))]
    LBrace,
    #[strum(props(regex = r#"^\}"#))]
    RBrace,
    #[strum(props(regex = r#"^\["#))]
    LBracket,
    #[strum(props(regex = r#"^\]"#))]
    RBracket,

    // Comparison
    #[strum(props(regex = r#"^=="#))]
    Equal,
    #[strum(props(regex = r#"^={3}"#))]
    TypeEqual,
    #[strum(props(regex = r#"^~="#))]
    NotEqual,
    #[strum(props(regex = r#"^<"#))]
    LessThan,
    #[strum(props(regex = r#"^>"#))]
    GreaterThan,
    // Other
    #[strum(props(regex = r#"^[a-zA-Z][\w\d]*"#))]
    Identifier,
    #[strum(props(regex = r#"^ +"#))]
    Space
}

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
            println!("Token type: {:?}", token_type);
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

pub fn test() {
    let input = r#"browtfx = 55.6"#;
    let re = regex::Regex::new(r#"^[a-zA-Z][\w\d]+"#).unwrap();
    let captures = re.captures(input).unwrap();
    println!("{:?}", captures);
}