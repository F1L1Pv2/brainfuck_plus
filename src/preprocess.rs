use std::process::exit;

// use std::process::exit;
use crate::common::*;


#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    pub tokens: Vec<Token>,
}

fn detect_macros(tokens: &Vec<Token>) -> (Vec<Macro>, Vec<Token>) {
    let mut i: usize = 0;
    let len: usize = tokens.len();
    let mut macros: Vec<Macro> = Vec::new();
    let mut next_tokens: Vec<Token> = Vec::new();
    while i < len {
        let token = tokens[i].clone();
        if token.token_type == TokenType::MacroDecl {
            if token.value == "#define" {
                i += 1;
                let mut macrom: Macro = Macro { name: String::new(), tokens: Vec::new() };

                let name: String = {
                    if tokens[i].token_type == TokenType::Ident {
                        tokens[i].value.clone()
                    } else {
                        println!("MACRO DECL: Expected Identifier");
                        exit(1);
                    }
                };

                macrom.name = name;

                i += 1;

                let mut terms: Vec<Token> = Vec::new();

                while tokens[i].token_type != TokenType::NewLine {
                    terms.push(tokens[i].clone());
                    i += 1;
                }

                macrom.tokens = terms;

                macros.push(macrom);
            } else {
                println!("This Macro doesnt exist `{}`",token.value);
                exit(1);
            }
        }else{
            next_tokens.push(token);
        }

        i += 1;

    }
    (macros, next_tokens)
}

fn is_macro(name: String, macros: &Vec<Macro>) -> Option<Macro>{
    let mut macrom: Option<Macro> = None;
    for macrom in macros.iter(){
        if name == macrom.name{
            println!("Not implemented yet");
            exit(1);
            break;
        }
    }
    macrom
}

fn unwrap_macro(uno_macro: Macro, macros: &Vec<Macro>)->Vec<Token>{
    let mut unwrap_token: Vec<Token> = Vec::new();
    for token in uno_macro.tokens.iter(){
        if token.token_type != TokenType::Ident {
            unwrap_token.push(token.clone());
        }else{
            let nmacro = is_macro(token.value, macros);
            if nmacro.is_some(){
                let nmacro = nmacro.unwrap();
                let mut tokens = unwrap_macro(nmacro, macros);
                unwrap_token.append(&mut tokens);
            }else{
                println!("Undentifier Not defined");
                exit(1);
            }
        }
    }
    unwrap_token
}

fn unwrap_macros(tokens: Vec<Token>, macros: Vec<Macro>)-> Vec<Token>{
    let mut next_tokens: Vec<Token> = Vec::new();
    let mut i: usize = 0;
    let len: usize = tokens.len();
    while i<len{

        let token = tokens[i];

        if !(token.token_type == TokenType::Ident){
            next_tokens.push(token);
        }

        let macrom = is_macro(token.value, &macros);
        if macrom.is_some(){
            let macrom = macrom.unwrap();
            let mut macro_tokens = unwrap_macro(macrom, &macros);
            next_tokens.append(&mut macro_tokens);
        }else{
            println!("Undentifier Not defined");
            exit(1);
        }


        i+=1;
        exit(1);
    }

    next_tokens
}

pub fn preprocess_tokens(tokens: Vec<Token>) -> Vec<Token> {
    // let mut new_tokens: Vec<Token> = Vec::new();

    let (macros, mut new_tokens) = detect_macros(&tokens);
    let new_tokens = unwrap_macros(tokens, macros);

    // for token in tokens.iter() {
    //     new_tokens.push(token.clone());
    // }

    new_tokens
}
