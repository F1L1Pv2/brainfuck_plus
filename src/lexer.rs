use std::process::exit;

use crate::common::*;

pub fn lex_file(contents: String) -> Vec<Token> {
    let mut comment_single = false;
    let mut comment_mul = false;
    let mut i: usize = 0;
    let len = contents.len();
    let mut tokens: Vec<Token> = Vec::new();
    while i < len {
        let ch = contents.chars().nth(i).unwrap();
        let next_char = if i + 1 < len {
            contents.chars().nth(i + 1).unwrap()
        } else {
            '\0'
        };
        let checker = ch.to_string() + next_char.to_string().as_str();

        if !comment_mul && !comment_single {
            match ch {
                '<' => {
                    tokens.push(Token {
                        token_type: TokenType::PointerLeft,
                        value: ch.to_string(),
                    });
                }
                '>' => {
                    tokens.push(Token {
                        token_type: TokenType::PointerRight,
                        value: ch.to_string(),
                    });
                }
                '&' => {
                    tokens.push(Token {
                        token_type: TokenType::PointerReset,
                        value: ch.to_string(),
                    });
                }
                '+' => {
                    tokens.push(Token {
                        token_type: TokenType::Add,
                        value: ch.to_string(),
                    });
                }
                '-' => {
                    tokens.push(Token {
                        token_type: TokenType::Sub,
                        value: ch.to_string(),
                    });
                }
                ',' => {
                    tokens.push(Token {
                        token_type: TokenType::ReadByte,
                        value: ch.to_string(),
                    });
                }
                '.' => {
                    tokens.push(Token {
                        token_type: TokenType::WriteByte,
                        value: ch.to_string(),
                    });
                }
                '\'' => {
                    tokens.push(Token {
                        token_type: TokenType::Clear,
                        value: ch.to_string(),
                    });
                }
                '%' => {
                    tokens.push(Token {
                        token_type: TokenType::BaseMemAddr,
                        value: ch.to_string(),
                    });
                }
                '$' => {
                    tokens.push(Token {
                        token_type: TokenType::MemAddr,
                        value: ch.to_string(),
                    });
                }
                '[' => {
                    tokens.push(Token {
                        token_type: TokenType::ZeroJump,
                        value: ch.to_string(),
                    });
                }
                ']' => {
                    tokens.push(Token {
                        token_type: TokenType::NonZeroJump,
                        value: ch.to_string(),
                    });
                }
                '?' => {
                    tokens.push(Token {
                        token_type: TokenType::Syscall,
                        value: ch.to_string(),
                    });
                }
                '\n' => {
                    tokens.push(Token {
                        token_type: TokenType::NewLine,
                        value: ch.to_string(),
                    });
                }

                _ => match checker.as_str() {
                    "//" => {
                        // println!("[Single Line Comment: ");
                        comment_single = true;
                        i += 2;
                        continue;
                    }
                    "/*" => {
                        // println!("[Multi Line Comment: ");
                        comment_mul = true;
                        i += 2;
                        continue;
                    }
                    _ => {
                        // println!("unexpected token: {}", ch);
                        if !ch.is_whitespace() {
                            if ch != '`'{
                                // println!("Unexpected token: {}", ch);
                                // println!("Idents and Macros not implemented yet");
                                let mut word: String = String::new();
                                // let nexty_ch = contents.chars().nth(i);
                                while contents.chars().nth(i).unwrap().is_alphabetic() || contents.chars().nth(i).unwrap() == '#'{
                                    // print!("{}", contents.chars().nth(i).unwrap());
                                    // println!("{}",contents.chars().nth(i).unwrap());
                                    word += contents.chars().nth(i).unwrap().to_string().as_str();
                                    i+=1;
                                    if contents.chars().nth(i).is_none(){
                                        break;
                                    }
                                }
                                // println!("");

                                // println!("Word: \"{}\"", word);
                                if word == "#define"{
                                    tokens.push(Token { token_type: TokenType::MacroDecl, value: word });
                                    continue;
                                }else{
                                    tokens.push(Token { token_type: TokenType::Ident, value: word });
                                    continue;
                                }

                            }else{
                                println!("Literals arent supported yet");
                                exit(1);
                            }
                        }
                    }
                },
            }
            i += 1;
        } else {
            // print!("{}", ch);

            if comment_mul && checker.as_str() == "*/" {
                comment_mul = false;
                comment_single = false;
                i += 2;
                tokens.push(Token {
                    token_type: TokenType::NewLine,
                    value: "\n".to_string(),
                });
                // println!("\nEnd of multi line comment]");
                continue;
            }

            if comment_single && (ch == '\n' || ch == '\0') {
                comment_mul = false;
                comment_single = false;
                tokens.push(Token {
                    token_type: TokenType::NewLine,
                    value: "\n".to_string(),
                });
                // println!("\nEnd of single line comment]");
            }
            i += 1;
        }
    }
    tokens
}
