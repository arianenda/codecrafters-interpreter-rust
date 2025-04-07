use crate::token::TokenType;
use std::collections::HashMap;
use std::sync::OnceLock;

static GLOBAL_MAP: OnceLock<HashMap<&'static str, TokenType>> = OnceLock::new();

pub fn init_keywords_map() -> &'static HashMap<&'static str, TokenType> {
    GLOBAL_MAP.get_or_init(|| {
        HashMap::from([
            ("and", TokenType::AND),
            ("class", TokenType::CLASS),
            ("else", TokenType::ELSE),
            ("false", TokenType::FALSE),
            ("for", TokenType::FOR),
            ("fun", TokenType::FUN),
            ("if", TokenType::IF),
            ("nil", TokenType::NIL),
            ("or", TokenType::OR),
            ("print", TokenType::PRINT),
            ("return", TokenType::RETURN),
            ("super", TokenType::SUPER),
            ("this", TokenType::THIS),
            ("true", TokenType::TRUE),
            ("var", TokenType::VAR),
            ("while", TokenType::WHILE),
        ])
    })
}
