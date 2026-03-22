/*
Copyright (C) 2026 GroboChan
Please see README.md and LICENSE.txt for more information
*/

use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Number(f64),
    Dice { count: u8, sides: u32 },
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponent,
    LeftBracket,
    RightBracket,
}

impl Token {
    pub fn precedence(self) -> i32 {
        match self {
            Token::Plus | Token::Minus => 1,
            Token::Multiply | Token::Divide => 2,
            Token::Exponent => 3,
            _ => 0,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Token::Dice { count, sides } => format!("{}d{}", *count, *sides),
            Token::Number(n) => format!("{}", *n),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Multiply => "*".to_string(),
            Token::Divide => "/".to_string(),
            Token::Exponent => "^".to_string(),
            Token::LeftBracket => "(".to_string(),
            Token::RightBracket => ")".to_string(),
        };

        write!(f, "{}", ch)
    }
}
