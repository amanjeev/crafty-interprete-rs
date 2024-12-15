use crate::tokens::Token;
use crate::tokens::TokenType;
use std::str::FromStr;
use crate::errors::Error;

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    line: usize,
}

impl Scanner {
    fn new(source: String) -> Scanner {
        let tokens = vec![];
        let line: usize = 1;

        Scanner {
            source,
            tokens,
            line,
        }
    }
    pub(crate) fn scan_tokens(&mut self) -> Result<(), Error> {
        let s = &self.source;

        for (item_num, character) in s.chars().enumerate() {
            let token = self.scan_token(&character)?;
            self.tokens.push(token);
        }

        // End of file.
        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        Ok(())
    }

    pub(crate) fn scan_token(&self, character: &char) -> Result<Token, Error> {
        let token_str = &character.to_string();
        let token_type = TokenType::from_str(token_str)?;
        Ok(Token::new(token_type, token_str.to_owned(), None, self.line))
    }
}
