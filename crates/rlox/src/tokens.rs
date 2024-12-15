use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::errors::Error;

#[derive(Debug)]
pub(crate) enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl FromStr for TokenType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t = match s {
            "(" => TokenType::LeftParen,
            ")" => TokenType::RightParen,
            "{" => TokenType::LeftBrace,
            "}" => TokenType::RightBrace,
            "," => TokenType::Comma,
            "." => TokenType::Dot,
            "-" => TokenType::Minus,
            "+" => TokenType::Plus,
            ";" => TokenType::Semicolon,
            "/" => TokenType::Slash,
            "*" => TokenType::Star,
            "!" => TokenType::Bang,
            "!=" => TokenType::BangEqual,
            "=" => TokenType::Equal,
            "==" => TokenType::EqualEqual,
            ">" => TokenType::Greater,
            ">=" => TokenType::GreaterEqual,
            "<" => TokenType::Less,
            "<=" => TokenType::LessEqual,
            // "Identifier" => TokenType::Identifier,
            // "String" => TokenType::String,
            // "Number" => TokenType::Number,
            // "And" => TokenType::And,
            // "Class" => TokenType::Class,
            // "Else" => TokenType::Else,
            // "False" => TokenType::False,
            // "Fun" => TokenType::Fun,
            // "For" => TokenType::For,
            // "If" => TokenType::If,
            // "Nil" => TokenType::Nil,
            // "Or" => TokenType::Or,
            // "Print" => TokenType::Print,
            // "Return" => TokenType::Return,
            // "Super" => TokenType::Super,
            // "This" => TokenType::This,
            // "True" => TokenType::True,
            // "Var" => TokenType::Var,
            // "While" => TokenType::While,
            // "Eof" => TokenType::Eof,
            _ => return Err(Error::UnexpectedCharacter),
        };

        Ok(t)
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub(crate) struct Literal;

#[derive(Debug)]
pub(crate) struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub(crate) fn new(
        ttype: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Self {
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {:?}", self.ttype, self.lexeme, self.literal)
    }
}
