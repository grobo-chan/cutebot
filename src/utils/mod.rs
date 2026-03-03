pub mod processor;
pub mod tokenizer;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Number(f64),
    Dice { count: u32, sides: u32 },
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftBracket,
    RightBracket,
}

impl Token {
    fn precedence(self) -> i32 {
        match self {
            Token::Plus | Token::Minus => 1,
            Token::Multiply | Token::Divide => 2,
            _ => 0,
        }
    }
}
