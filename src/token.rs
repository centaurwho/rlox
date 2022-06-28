// TODO: Better hierarchy
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum TokenType {
    // Single character tokens
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

    // One or Two character tokens
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

    // Special
    Eof,
}

#[derive(Debug)]
pub enum Literal {
    None,
    String(String),
    Number(u32),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: u32) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_show() {
        let t = Token::new(TokenType::Identifier, String::from("9"), Literal::Number(9), 12);
        println!("{:?}", t);
    }
}

