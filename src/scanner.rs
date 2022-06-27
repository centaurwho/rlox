use crate::token::{Literal, Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    // Not a mutable self reference since we won't use Scanner ever again
    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::Eof, String::from(""), Literal::None, self.line));
        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_nonliteral(TokenType::LeftParen),
            ')' => self.add_nonliteral(TokenType::RightParen),
            '{' => self.add_nonliteral(TokenType::LeftBrace),
            '}' => self.add_nonliteral(TokenType::RightBrace),
            ',' => self.add_nonliteral(TokenType::Comma),
            '.' => self.add_nonliteral(TokenType::Dot),
            '-' => self.add_nonliteral(TokenType::Minus),
            '+' => self.add_nonliteral(TokenType::Plus),
            ';' => self.add_nonliteral(TokenType::Semicolon),
            '*' => self.add_nonliteral(TokenType::Star),
            // TODO: dont panic
            _ => panic!("oof"),
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        ch
    }

    fn add_nonliteral(&mut self, token_type: TokenType) {
        self.add_token(token_type, Literal::None);
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, String::from(text), literal, self.line));
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::Scanner;

    #[test]
    fn scan_tokens() {
        let scanner = Scanner::new(String::from(";,{{"));
        let vec = scanner.scan_tokens();
        println!("{:?}", vec);
    }
}