use crate::token::{Literal, Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    cursor: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            cursor: 0,
            line: 1,
        }
    }

    // Not a mutable self reference since we won't use Scanner ever again
    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.cursor;
            self.scan_token();
        }
        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            Literal::None,
            self.line,
        ));
        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.cursor >= self.source.len()
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
            '!' => {
                let tt = self.match_cond('=', TokenType::BangEqual, TokenType::Bang);
                self.add_nonliteral(tt);
            }
            '=' => {
                let tt = self.match_cond('=', TokenType::EqualEqual, TokenType::Equal);
                self.add_nonliteral(tt);
            }
            '<' => {
                let tt = self.match_cond('=', TokenType::LessEqual, TokenType::Less);
                self.add_nonliteral(tt)
            }
            '>' => {
                let tt = self.match_cond('=', TokenType::GreaterEqual, TokenType::Greater);
                self.add_nonliteral(tt);
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_nonliteral(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            // TODO: dont panic
            _ => panic!("oof"),
        }
    }

    fn current_char(&self) -> char {
        self.source.chars().nth(self.cursor).unwrap()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.current_char()
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.current_char();
        self.cursor += 1;
        ch
    }

    fn match_cond(&mut self, character: char, tt1: TokenType, tt2: TokenType) -> TokenType {
        if self.match_char(character) {
            tt1
        } else {
            tt2
        }
    }

    fn match_char(&mut self, character: char) -> bool {
        if self.is_at_end() || self.current_char() != character {
            return false;
        }
        self.cursor += 1;
        true
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        // Unterminated string
        if self.is_at_end() {
            // TODO error handling
            return;
        }
        // Closing "
        self.advance();

        // TODO: cleanup
        let val = &self.source.as_str()[self.start+1..self.cursor-1];
        self.add_token(TokenType::String, Literal::String(String::from(val)));
    }

    fn add_nonliteral(&mut self, token_type: TokenType) {
        self.add_token(token_type, Literal::None);
    }
    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = &self.source[self.start..self.cursor];
        self.tokens.push(Token::new(
            token_type,
            String::from(text),
            literal,
            self.line,
        ));
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
