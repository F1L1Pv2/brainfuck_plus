// use std::process::exit;
use crate::common::*;

fn trim_tokens(i: &mut usize, tokens: &Vec<Token>, token_type: TokenType) -> Operation {
    
    let mut count = 0;
    
    let len: usize = tokens.len();

    while tokens[*i].token_type == token_type{
        count += 1;
        *i+=1;
        if *i>=len{
            break;
        }
    }

    Operation { token_type, count }
}

pub fn parse_file(tokens: Vec<Token>) -> Vec<Operation> {
    let mut operations: Vec<Operation> = Vec::new();

    let mut i: usize = 0;
    let len: usize = tokens.len();

    while i < len {
        let token = tokens[i].clone();

        match token.token_type {
            TokenType::Add => {
                operations.push(trim_tokens(&mut i, &tokens, token.token_type))
            }

            TokenType::Sub => {
                operations.push(trim_tokens(&mut i, &tokens, token.token_type))
            }

            TokenType::PointerLeft => {
                operations.push(trim_tokens(&mut i, &tokens, token.token_type))
            }

            TokenType::PointerRight => {
                operations.push(trim_tokens(&mut i, &tokens, token.token_type))
            }

            TokenType::Push => {
                operations.push(trim_tokens(&mut i, &tokens, token.token_type))
            }

            TokenType::Pop => {
                operations.push(trim_tokens(&mut i, &tokens, token.token_type))
            }

            _ => {
                operations.push(Operation { token_type: token.token_type, count: 1 });
                i += 1;
            }
        }

    }

    operations
}
