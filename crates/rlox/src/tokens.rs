use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum TokenType {
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

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct Literal;

#[derive(Debug)]
struct Token {
    ttype: TokenType,
    lexeme: String,
    line: usize,
    literal: Literal,
}

impl Token {
    fn new(ttype: TokenType, lexeme: String, line: usize, literal: Literal) -> Self {
        Token {
            ttype,
            lexeme,
            line,
            literal,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {:?}", self.ttype, self.lexeme, self.literal)
    }
}
