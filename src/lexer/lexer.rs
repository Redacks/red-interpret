use regex::Regex;

use super::{Token, TokenType};

pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer { input }
    }

    fn get_preprocess_input(&self) -> Vec<String> {
        let global_var_pattern = Regex::new(r"(= )?(\$[_a-zA-Z][_a-zA-Z0-9]*\$)(\s*\n)?").unwrap(); //Used to fix spacing around variable
        let new_line_separator = Regex::new(r"\n").unwrap(); //Used to split by line

        let mut splitting_newlines = new_line_separator
            .replace_all(self.input.trim_end_matches("\n"), " \n ")
            .to_string();
        splitting_newlines.push_str(" \n");

        let result =
            global_var_pattern.replace_all(&splitting_newlines, |caps: &regex::Captures| {
                if caps.get(1).is_some() && caps.get(2).is_some() && caps.get(3).is_some() {
                    println!("{:?}", caps);
                    (caps[0].trim().to_string() + " \n").to_string()
                } else {
                    format!(" {} ", caps.get(2).unwrap().as_str())
                }
            });
        result.split(" ").map(|x| x.to_owned()).collect()
    }

    pub fn lex(&self) -> Vec<Token> {
        let identifier_pattern = Regex::new(r"^[_a-zA-Z][_a-zA-Z0-9]*$").unwrap(); //Used to identify variable names after Instruction
        let var_pattern = Regex::new(r"^\$([a-zA-Z_$][\w$]*)\$$").unwrap(); //Used to match var inside of a string value

        let processed_input = self.get_preprocess_input();
        let mut tokens: Vec<Token> = Vec::new();
        let mut last_token = Token::new(0, TokenType::NEWLINE);
        let mut current_line = 1;

        for token in processed_input {
            let lexed_token = match last_token.token_type {
                TokenType::NEWLINE => match token.as_str() {
                    "Zahl" => Token::new(current_line, TokenType::ZAHL),
                    "Text" => Token::new(current_line, TokenType::TEXT),
                    "Input" => Token::new(current_line, TokenType::INPUT),
                    "Output" => Token::new(current_line, TokenType::OUTPUT),
                    _ => Token::new(current_line, TokenType::INVALID(token)),
                },
                TokenType::INPUT | TokenType::OUTPUT | TokenType::TEXT | TokenType::ZAHL => {
                    if identifier_pattern.is_match(token.as_str()) {
                        Token::new(current_line, TokenType::IDENTIFIER(token))
                    } else {
                        Token::new(current_line, TokenType::INVALID(token))
                    }
                }
                TokenType::IDENTIFIER(_) => {
                    if token == "=" {
                        Token::new(current_line, TokenType::EQUAL)
                    } else if token == "\n" {
                        current_line += 1;
                        Token::new(current_line, TokenType::NEWLINE)
                    } else if let Some(res) = var_pattern.captures(token.as_str()) {
                        Token::new(
                            current_line,
                            TokenType::IDENTIFIER(res.get(1).unwrap().as_str().to_owned()),
                        )
                    } else {
                        Token::new(current_line, TokenType::VALUE(token))
                    }
                }
                _ => {
                    if token == "\n" {
                        current_line += 1;
                        Token::new(current_line, TokenType::NEWLINE)
                    } else if let Some(res) = var_pattern.captures(token.as_str()) {
                        Token::new(
                            current_line,
                            TokenType::IDENTIFIER(res.get(1).unwrap().as_str().to_owned()),
                        )
                    } else {
                        Token::new(current_line, TokenType::VALUE(token))
                    }
                }
            };
            last_token = lexed_token.clone();
            tokens.push(lexed_token);
        }

        tokens
    }
}
