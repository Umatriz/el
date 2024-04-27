use crate::tokens::{Token, TokenKind};

#[derive(Default)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            ..Default::default()
        }
    }

    pub fn scan2(source: String) -> Vec<(String, TokenKind)> {
        let len = source.len();
        let mut chars = source.chars().map(String::from).collect::<Vec<String>>();

        let mut tokens = Vec::new();

        for index in 0..len {
            let kind = Self::scan_token(dbg!(&chars), index);
            tokens.push((chars[index].clone(), kind));
        }

        tokens
    }

    fn scan_token(chars: &[String], index: usize) -> TokenKind {
        use TokenKind as T;
        match dbg!(chars[index].as_str()) {
            "(" => T::LeftParen,
            ")" => T::RightParen,
            "{" => T::LeftBrace,
            "}" => T::RightBrace,
            "+" => T::Plus,
            "-" => T::Minus,
            "," => T::Comma,
            "." => T::Dot,
            ";" => T::Semicolon,
            "*" => T::Star,
            _ => unreachable!(),
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current as usize >= self.source.len()
    }

    pub fn scan(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;

            // self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenKind::EOF, "".into(), self.line));

        &self.tokens
    }
}
