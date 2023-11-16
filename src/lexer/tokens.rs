#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    TEXT,
    ZAHL,
    EQUAL,
    VALUE,
    OUTPUT,
    INPUT,
    IDENTIFIER,
    NEWLINE,
    EOF,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub token_type: TokenType,
    pub value: String,
}
impl Token {
    pub fn new(
        line: usize,
        start: usize,
        end: usize,
        value: String,
        token_type: TokenType,
    ) -> Self {
        Token {
            line,
            token_type,
            start,
            end,
            value,
        }
    }
}
