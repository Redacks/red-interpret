#[derive(Clone, Debug)]
pub enum TokenType {
    TEXT,
    ZAHL,
    EQUAL,
    VALUE(String),
    OUTPUT,
    INPUT,
    IDENTIFIER(String),
    NEWLINE,
    INVALID(String),
}

#[derive(Clone, Debug)]
pub struct Token {
    pub line: usize,
    pub token_type: TokenType,
}
impl Token {
    pub fn new(line: usize, token_type: TokenType) -> Self {
        Token { line, token_type }
    }
}
