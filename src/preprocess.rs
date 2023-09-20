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

pub fn is_macro(name: String, macros: &[Macro]) -> Option<Macro>{
    let mut macroms: Option<Macro> = None;
    for macrom in macros.iter(){
        if name == macrom.name{
            macroms = Some(macrom.clone());
            break;
        }
    }
    macroms
}

fn unwrap_macro(uno_macro: Macro, macros: &Vec<Macro>)->Vec<Token>{
    let mut unwrap_token: Vec<Token> = Vec::new();
    for token in uno_macro.tokens.iter(){
        if token.token_type != TokenType::Ident {
            unwrap_token.push(token.clone());
        }else{
            let name = token.value.clone();
            let nmacro = is_macro(name, macros);
            if let Some(nmacro) = nmacro{
            // if nmacro.is_some(){
                // let nmacro = nmacro.unwrap();
                let mut tokens = unwrap_macro(nmacro, macros);
                unwrap_token.append(&mut tokens);
            }else{
                println!("Unwrap Macros: Undentifier Not defined {}", token.value);
                exit(1);
            }
        }
    }
    unwrap_token
}

fn ifdefs_macros(tokens: Vec<Token>, macros: Vec<Macro>)-> Vec<Token>{
    let mut next_tokens: Vec<Token> = Vec::new();
    let mut i: usize = 0;
    let len: usize = tokens.len();
    while i<len{
        let token = tokens[i].clone();

        match token.token_type{

            TokenType::IfdefMacro => {
                let mut base_tokens: Vec<Token> = Vec::new();
                let mut else_tokens: Vec<Token> = Vec::new();

                let mut base = true;

                i+= 1;

                let name = tokens[i].clone();

                    i+=1;

                    while tokens[i].token_type != TokenType::EndifMacro {

                        let cur_token = tokens[i].clone();
                        
                        if cur_token.token_type != TokenType::ElseMacro{


                            if base{
                                base_tokens.push(cur_token);
                            }else{
                                else_tokens.push(cur_token);
                            }

                        }else{
                            if base{
                                base = false;
                            }else{
                                println!("#else was already defined");
                                exit(1);
                            }
                        }
                        i+=1;
                    }
                
                if is_macro(name.value, &macros).is_some(){
                    next_tokens.append(&mut base_tokens);
                }else{
                    next_tokens.append(&mut else_tokens);
                }

                i+=1;

                // dbg!(next_tokens);

                // println!("Not implemented yet");
                // exit(1);
            }
            TokenType::IfNdefMacro => {
                let mut base_tokens: Vec<Token> = Vec::new();
                let mut else_tokens: Vec<Token> = Vec::new();

                let mut base = true;

                i+= 1;

                let name = tokens[i].clone();

                    i+=1;

                    while tokens[i].token_type != TokenType::EndifMacro {

                        let cur_token = tokens[i].clone();
                        
                        if cur_token.token_type != TokenType::ElseMacro{


                            if base{
                                base_tokens.push(cur_token);
                            }else{
                                else_tokens.push(cur_token);
                            }

                        }else{
                            if base{
                                base = false;
                            }else{
                                println!("#else was already defined");
                                exit(1);
                            }
                        }
                        i+=1;
                    }
                
                if is_macro(name.value, &macros).is_none(){
                    next_tokens.append(&mut base_tokens);
                }else{
                    next_tokens.append(&mut else_tokens);
                }

                i+=1;

                // dbg!(&next_tokens);

                // println!("Not implemented yet");
                // exit(1);
            }
            TokenType::ElseMacro => {
                println!("You need to specify condition");
                exit(1);
            }
            TokenType::EndifMacro => {
                println!("You need to specify condition");
                exit(1);
            }

            TokenType::MacroDecl =>{
                println!("Unreachable. Something is wrong with detecting macros");
                exit(1);
            }

            _ => {
                next_tokens.push(token);
                i+=1;
            }
        }


    }

    next_tokens
}

fn unwrap_macros(tokens: Vec<Token>, macros: Vec<Macro>)-> Vec<Token>{
    let mut next_tokens: Vec<Token> = Vec::new();
    let mut i: usize = 0;
    let len: usize = tokens.len();
    while i<len{

        let token = tokens[i].clone();

        if !(token.token_type == TokenType::Ident){
            next_tokens.push(token);
        }else{

        let name = token.value.clone();
        let macrom = is_macro(name.clone(), &macros);
        // if macrom.is_some(){
        if let Some(macrom) = macrom {
            // let macrom = macrom.unwrap();
            let mut macro_tokens = unwrap_macro(macrom, &macros);
            next_tokens.append(&mut macro_tokens);
        }else{
            println!("Undentifier Not defined: {}", name);
            exit(1);
        }

        }


        i+=1;
    }

    next_tokens
}

pub fn preprocess_tokens(tokens: Vec<Token>) -> Vec<Token> {
    // let mut new_tokens: Vec<Token> = Vec::new();

    //TODO: currently you cannot have conditional macros fix it
    /*
        #define test

        #ifdef test
            #define color `"red"`
        #else
            #define color `"blue"`
        #endif

        this wont work `color` whill always be "red"

     */

    //TODO: Add ability to pass arguments into macros
    /*
    
        #include <stdlib.bf>

        #define car(color | type) write(color) `"\n"`. write(type) `"\n"`.

        car(`"red"` | `"toyota"`)

     */

    let (macros, new_tokens) = detect_macros(&tokens);
    
    let new_tokens = ifdefs_macros(new_tokens, macros.clone());
    
    unwrap_macros(new_tokens, macros)

    // println!("--------------------");

    // dbg!(&new_tokens);

    // exit(1);

    // for token in tokens.iter() {
    //     new_tokens.push(token.clone());
    // }

    // new_tokens
}
