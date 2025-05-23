use ::std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
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
    STRING,
    NUMBER,
    IDENTIFIER,

    //keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EQUAL_EQUAL,
    BANG_EQUAL,
    LESS_EQUAL,
    GREATER_EQUAL,

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub _type: TokenType,
    pub _string: String,
    pub _value: Option<String>,
}

impl Token {
    pub fn new(_type: TokenType, _string: String) -> Self {
        Token {
            _type,
            _string,
            _value: None,
        }
    }

    pub fn new_with_value(_type: TokenType, _string: String, _value: Option<String>) -> Self {
        Token {
            _type,
            _string,
            _value: _value,
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
