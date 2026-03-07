use crate::utils::token::Token;
use crate::utils::tokenizer::tokenizer;
use rand::prelude::*;

async fn dice_to_num(dice: Token) -> Token {
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

async fn convert_to_rpn(input: Vec<Token>) -> Vec<Token> {
    let mut output_queue: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();

    for token in input {
        match token {
            Token::Dice { .. } => {
                output_queue.push(dice_to_num(token).await);
            }

            Token::Number(_) => {
                output_queue.push(token);
            }

            Token::LeftBracket => {
                operator_stack.push(token);
            }

            Token::RightBracket => {
                while let Some(top_token) = operator_stack.last() {
                    if top_token == &Token::LeftBracket {
                        break;
                    }

                    output_queue.push(operator_stack.pop().expect("Mismatched Brackets"));
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
                        output_queue.push(operator_stack.pop().expect("Something went wrong"));
                    } else {
                        break;
                    }
                }

                operator_stack.push(token);
            }
        }
    }

    while let Some(top_token) = operator_stack.last() {
        if top_token == &Token::LeftBracket {
            panic!("Mismatched Brackets");
        }

        output_queue.push(operator_stack.pop().expect("Something went wrong"));
    }

    output_queue
}

fn print_token_vec(input: &Vec<Token>) {
    for token in input {
        print!("{} ", token);
    }
    println!();
}

pub async fn processor(expr: &str) -> f64 {
    let input = tokenizer(expr).await;
    print!("Expression: ");
    print_token_vec(&input);
    let rpn = convert_to_rpn(input).await;
    print!("RPN: ");
    print_token_vec(&rpn);
    let mut stack: Vec<f64> = Vec::new();

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
            Token::Exponent => {
                let a = stack.pop().expect("Invalid RPN!");
                let b = stack.pop().expect("Invalid RPN!");
                stack.push(b.powf(a));
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
