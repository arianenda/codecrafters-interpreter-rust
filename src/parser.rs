use crate::expr::Expr;
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Parser {
    token: Vec<Token>,
    curr_pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            token: tokens,
            curr_pos: 0,
        }
    }

    fn advance(&mut self) -> bool {
        if self.curr_pos + 1 < self.token.len() {
            self.curr_pos += 1;
            true
        } else {
            false
        }
    }

    fn expression(&mut self) -> Option<Expr> {
        if self.curr_pos >= self.token.len() {
            return None;
        }
        self.primary()
    }

    fn primary(&mut self) -> Option<Expr> {
        let curr_token = &self.token[self.curr_pos];
        let expr = match curr_token._type {
            TokenType::FALSE | TokenType::TRUE => {
                let value = curr_token._string.clone();
                self.advance();
                Some(Expr::Boolean(value))
            }
            TokenType::NUMBER => {
                let value = curr_token._value.clone()?;
                self.advance();
                Some(Expr::Number(value))
            }
            TokenType::NIL => {
                self.advance();
                Some(Expr::Nil)
            }
            _ => None,
        };
        expr
    }
}

pub fn parse(tokens: Vec<Token>) -> Option<Expr> {
    if tokens.is_empty() {
        return None;
    }
    let mut parser = Parser::new(tokens);
    parser.expression()
}
