use std::fmt;

#[derive(Debug)]
pub enum Expr {
    Boolean(String),
    Number(String),
    String(String),
    Nil,
}

impl Expr {
    pub fn print_token_value(&self) -> String {
        match self {
            Expr::Boolean(bool) => bool.to_string(),
            Expr::Number(num) => num.to_string(),
            Expr::String(str) => str.to_string(),
            Expr::Nil => "nil".to_string(),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Boolean(bool) => write!(f, "{}", bool),
            Expr::Number(num) => write!(f, "{}", num),
            Expr::String(str) => write!(f, "{}", str),
            Expr::Nil => write!(f, ""),
        }
    }
}
