use super::Token;
use rand::RngExt;
use std::collections::VecDeque;

fn dice_to_num(dice: Token) -> Token {
    if let Token::Dice { count, sides } = dice {
        let mut s: f64 = 0.0;
        let mut rng = rand::rng();

        for _ in 0..count {
            s += rng.random_range(1..=sides) as f64;
        }

        return Token::Number(s);
    } else {
        Token::Number(0.0)
    }
}

fn convert_to_rpn(input: Vec<Token>) -> Vec<Token> {
    let mut output_queue: VecDeque<Token> = VecDeque::new();
    let mut operator_stack: Vec<Token> = Vec::new();
    let mut tokens = input.iter().peekable();

    while let Some(&token) = tokens.peek() {
        match token {
            Token::Dice { count, sides } => {
                output_queue.push_front(dice_to_num(Token::Dice {
                    count: *count,
                    sides: *sides,
                }));
            }

            Token::Number(n) => {
                output_queue.push_front(Token::Number(*n));
            }

            Token::LeftBracket => {
                operator_stack.push(Token::LeftBracket);
            }

            Token::RightBracket => {
                while let Some(top_token) = operator_stack.last() {
                    if top_token == &Token::LeftBracket {
                        break;
                    }

                    output_queue.push_front(operator_stack.pop().expect("Mismatched Brackets"));
                }

                if operator_stack.pop().is_none() {
                    panic!("Mismatched Brackets!");
                }
            }

            _ => {
                while let Some(top_token) = operator_stack.last() {
                    if top_token != &Token::LeftBracket
                        && top_token.precedence() >= token.precedence()
                    {
                        output_queue
                            .push_front(operator_stack.pop().expect("Something went wrong"));
                    }
                }

                operator_stack.push(*token);
            }
        }

        tokens.next();
    }

    while let Some(top_token) = operator_stack.last() {
        if top_token == &Token::LeftBracket {
            panic!("Mismatched Brackets");
        }

        output_queue.push_front(operator_stack.pop().expect("Something went wrong"));
    }

    let mut output_vec = Vec::from(output_queue);
    output_vec.reverse();
    output_vec
}

pub fn processor(input: Vec<Token>) -> f64 {
    let rpn = convert_to_rpn(input);
    let mut stack: Vec<f64> = Vec::new();

    // for token in &rpn {
    //     let char = match token {
    //         Token::Number(n) => format!("{}", n),
    //         Token::Plus => String::from("+"),
    //         Token::Minus => String::from("-"),
    //         Token::Multiply => String::from("*"),
    //         Token::Divide => String::from("/"),
    //         _ => String::from("@"),
    //     };
    //     print!("{} ", char);
    // }
    // println!();

    for token in rpn {
        match token {
            Token::Number(n) => {
                stack.push(n);
            }
            Token::Plus => {
                let a = stack.pop().expect("Invalid RPN!");
                let b = stack.pop().expect("Invalid RPN!");
                stack.push(b + a);
            }
            Token::Minus => {
                let a = stack.pop().expect("Invalid RPN!");
                let b = stack.pop().expect("Invalid RPN!");
                stack.push(b - a);
            }
            Token::Multiply => {
                let a = stack.pop().expect("Invalid RPN!");
                let b = stack.pop().expect("Invalid RPN!");
                stack.push(b * a);
            }
            Token::Divide => {
                let a = stack.pop().expect("Invalid RPN!");
                let b = stack.pop().expect("Invalid RPN!");
                stack.push(b / a);
            }
            _ => {
                panic!("Invalid RPN!");
            }
        }
    }

    if stack.len() != 1 {
        panic!("Invalid RPN!");
    }

    return stack[0];
}
