use crate::token::Token;

pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source
        }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        vec![]
    }
}