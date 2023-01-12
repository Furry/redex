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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumProperty, EnumIter)]
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

impl TokenType {
    pub fn is_math(&self) -> bool {
        match self {
            TokenType::Plus | TokenType::Minus | TokenType::Asterisk | TokenType::Slash => true,
            _ => false,
        }
    }

    pub fn is_comparison(&self) -> bool {
        match self {
            TokenType::Equal | TokenType::TypeEqual | TokenType::NotEqual | TokenType::LessThan | TokenType::GreaterThan => true,
            _ => false,
        }
    }

    pub fn is_literal(&self) -> bool {
        match self {
            TokenType::IntegerLiteral | TokenType::BooleanLiteral | TokenType::StringLiteral => true,
            _ => false,
        }
    }
}

pub fn test() {
    let input = r#"browtfx = 55.6"#;
    let re = regex::Regex::new(r#"^[a-zA-Z][\w\d]+"#).unwrap();
    let captures = re.captures(input).unwrap();
    println!("{:?}", captures);
}