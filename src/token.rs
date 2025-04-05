use ::std::fmt::Display;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    PLUS,
    MINUS,
    SEMICOLON,
    STAR,
    EQUAL,
    BANG,
    LESS,
    GREATER,
    SLASH,

    EQUAL_EQUAL,
    BANG_EQUAL,
    LESS_EQUAL,
    GREATER_EQUAL,

    EOF,
}

pub struct Token {
    _type: TokenType,
    _string: String,
    _value: Option<String>,
}

impl Token {
    pub fn new(_type: TokenType, _string: String) -> Self {
        Token {
            _type,
            _string,
            _value: None,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self._type,
            self._string,
            self._value.clone().unwrap_or("null".to_string())
        )
    }
}
