use crate::tokens::Token;
use crate::tokens::TokenType;
struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: String) -> Scanner {
        let start: usize = 0;
        let current: usize = 0;
        let line: usize = 1;

        Scanner {
            source,
            start,
            current,
            line,
        }
    }

    pub(crate) fn scan_tokens(&mut self) -> Vec<Token> {
        let s = &self.source;
        let mut tokens: Vec<Token> = vec![];

        for (item_num, character) in s.chars().enumerate() {
            self.start = self.current;
            let token = self.scan_token();
            tokens.push(token);
        }

        tokens
    }

    pub(crate) fn scan_token(&self) -> Token {
        unimplemented!()
    }
}
