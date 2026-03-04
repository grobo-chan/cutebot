use super::Token;

async fn consume_numbers(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> f64 {
    let mut number_str = String::new();
    let mut has_decimal = false;

    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            number_str.push(chars.next().unwrap());
        } else if c == '.' && !has_decimal {
            has_decimal = true;
            number_str.push(chars.next().unwrap());
        } else {
            break;
        }
    }

    number_str.parse().unwrap_or(0.0)
}

pub async fn tokenizer(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' | '.' => {
                let first_num = consume_numbers(&mut chars).await;

                if chars.peek() == Some(&'d') {
                    chars.next();
                    let sides = consume_numbers(&mut chars).await;
                    let dice = Token::Dice {
                        count: first_num as u8,
                        sides: sides as u32,
                    };
                    tokens.push(dice);
                } else {
                    tokens.push(Token::Number(first_num));
                }
            }
            'd' => {
                chars.next();
                let sides = consume_numbers(&mut chars).await;
                let dice = Token::Dice {
                    count: 1 as u8,
                    sides: sides as u32,
                };
                tokens.push(dice);
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Multiply);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Divide);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftBracket);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightBracket);
                chars.next();
            }
            '^' => {
                tokens.push(Token::Exponent);
                chars.next();
            }
            ' ' | '\n' | '\t' => {
                chars.next();
            }
            _ => panic!("Unexpected character {}", c),
        }
    }

    return tokens;
}
