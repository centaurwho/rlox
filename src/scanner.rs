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

    // Not borrowing since we won't use Scanner after this call
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
            // TODO: Support for floats like .4 .5112
            '.' => self.add_nonliteral(TokenType::Dot),
            '-' => self.add_nonliteral(TokenType::Minus),
            // TODO: Support for negative numbers
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
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,
            '"' => self.string(),
            c => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    // TODO: dont panic
                    panic!("oof");
                }
            }
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
        let val = &self.source.as_str()[self.start + 1..self.cursor - 1];
        let literal = Literal::String(String::from(val));
        self.add_token(TokenType::String, literal);
    }

    fn add_nonliteral(&mut self, token_type: TokenType) {
        self.add_token(token_type, Literal::None);
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = String::from(self.extract_str());
        self.tokens.push(Token::new(
            token_type,
            text,
            literal,
            self.line,
        ));
    }

    fn is_digit(&self, ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    fn is_alpha(&self, ch: char) -> bool {
        (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '_'
    }

    fn is_alphanumeric(&self, ch: char) -> bool {
        self.is_alpha(ch) || self.is_digit(ch)
    }

    // TODO: Support fractions .3 and 41.
    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
            let val = self.extract_str().parse::<f32>().unwrap();
            self.add_token(TokenType::Float, Literal::Float(val));
        } else {
            let val = self.extract_str().parse::<u32>().unwrap();
            self.add_token(TokenType::Integer, Literal::Integer(val));
        };
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }
        let text = self.extract_str();
        let tt = TokenType::from_keyword(text);
        self.add_nonliteral(tt);
    }

    fn extract_str(&mut self) -> &str {
        &self.source[self.start..self.cursor]
    }
}

// TODO: More tests in detail
#[cfg(test)]
mod tests {
    use crate::scanner::{Scanner, KEYWORDS};

    #[test]
    fn scan_tokens() {
        let scanner = Scanner::new(String::from(";,{{"));
        let vec = scanner.scan_tokens();
        println!("{:?}", vec);
    }
}
