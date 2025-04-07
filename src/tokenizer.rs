use anyhow::bail;
use std::fs;

use crate::error::Error;
use crate::identifier::is_alpha_numeric;
use crate::number::{format_decimal, is_digit};
use crate::token::{Token, TokenType};

pub fn tokenize(filename: &String) -> anyhow::Result<()> {
    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => bail!(Error::new(u8::MAX)),
    };

    let mut unexpected_char_err = false;
    let mut string_literals_error = false;
    let mut line = 1usize;
    let mut chars = file_contents.chars();
    let mut tokens = vec![];

    while let Some(c) = chars.next() {
        match c {
            '(' => tokens.push(Token::new(TokenType::LEFT_PAREN, c.to_string())),
            ')' => tokens.push(Token::new(TokenType::RIGHT_PAREN, c.to_string())),
            '{' => tokens.push(Token::new(TokenType::LEFT_BRACE, c.to_string())),
            '}' => tokens.push(Token::new(TokenType::RIGHT_BRACE, c.to_string())),
            ',' => tokens.push(Token::new(TokenType::COMMA, c.to_string())),
            '.' => tokens.push(Token::new(TokenType::DOT, c.to_string())),
            '+' => tokens.push(Token::new(TokenType::PLUS, c.to_string())),
            '-' => tokens.push(Token::new(TokenType::MINUS, c.to_string())),
            ';' => tokens.push(Token::new(TokenType::SEMICOLON, c.to_string())),
            '*' => tokens.push(Token::new(TokenType::STAR, c.to_string())),
            '=' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('=') {
                    chars.next();
                    tokens.push(Token::new(TokenType::EQUAL_EQUAL, "==".to_string()));
                } else {
                    tokens.push(Token::new(TokenType::EQUAL, c.to_string()));
                }
            }
            '!' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('=') {
                    chars.next();
                    tokens.push(Token::new(TokenType::BANG_EQUAL, "!=".to_string()));
                } else {
                    tokens.push(Token::new(TokenType::BANG, c.to_string()));
                }
            }
            '<' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('=') {
                    chars.next();
                    tokens.push(Token::new(TokenType::LESS_EQUAL, "<=".to_string()));
                } else {
                    tokens.push(Token::new(TokenType::LESS, c.to_string()));
                }
            }
            '>' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('=') {
                    chars.next();
                    tokens.push(Token::new(TokenType::GREATER_EQUAL, ">=".to_string()));
                } else {
                    tokens.push(Token::new(TokenType::GREATER, c.to_string()));
                }
            }
            '/' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('/') {
                    while let Some(c) = chars.next() {
                        if c == '\n' {
                            line += 1;
                            break;
                        }
                    }
                } else {
                    tokens.push(Token::new(TokenType::SLASH, c.to_string()));
                }
            }
            ' ' | '\t' => {
                continue;
            }
            '\n' => {
                line += 1;
                continue;
            }
            '"' => {
                let mut strings = String::new();
                let mut strings_closed = false;
                while let Some(c) = chars.next() {
                    match c {
                        '"' => {
                            let str_contents = strings.clone();
                            let format_str = format!("\"{}\"", str_contents);
                            tokens.push(Token::new_with_value(
                                TokenType::STRING,
                                format_str,
                                Some(str_contents),
                            ));
                            strings_closed = true;
                            break;
                        }
                        _ => {
                            strings.push(c);
                        }
                    }
                }
                if !strings_closed {
                    eprintln!("[line {}] Error: Unterminated string.", line);
                    string_literals_error = true;
                }
            }
            '0'..='9' => {
                let mut numbers = String::new();
                let mut peekable = chars.clone().peekable();
                numbers.push(c);
                while let Some(c) = peekable.next() {
                    if is_digit(c) {
                        numbers.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::new_with_value(
                    TokenType::NUMBER,
                    numbers.to_string(),
                    Some(format_decimal(&numbers)),
                ));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifiers = String::new();
                let mut peekable = chars.clone().peekable();
                identifiers.push(c);

                while let Some(c) = peekable.next() {
                    if is_alpha_numeric(c) {
                        identifiers.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token::new(TokenType::IDENTIFIER, identifiers.to_string()))
            }
            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", line, c);
                unexpected_char_err = true;
            }
        }
    }

    tokens.push(Token::new(TokenType::EOF, "".to_string()));

    for token in tokens {
        println!("{}", token);
    }
    return if unexpected_char_err | string_literals_error {
        bail!(Error::new(65))
    } else {
        Ok(())
    };
}
