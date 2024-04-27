pub struct Token {
    kind: TokenKind,
    lexeme: String,
    line: u32,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: u32) -> Self {
        Self { kind, lexeme, line }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Minus,
    Plus,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Struct,
    Else,
    True,
    False,
    Function,
    For,
    If,
    Or,
    Print,
    Return,
    StructSelf,
    Let,
    While,

    EOF,
}
