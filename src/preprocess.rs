use std::process::exit;

// use std::process::exit;
use crate::common::*;


#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    pub tokens: Vec<Token>,
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
        if let Some(macrom) = macrom {
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

fn is_macro_token(token_type: TokenType) -> bool{

    token_type == TokenType::MacroDecl ||
    token_type == TokenType::IfdefMacro ||
    token_type == TokenType::IfNdefMacro ||
    token_type == TokenType::ElseMacro ||
    token_type == TokenType::EndifMacro 

}

fn preprocess_macro_decl(i: &mut usize, tokens: &[Token], macros: &mut Vec<Macro>){
    
    let mut macrom: Macro = Macro{name: String::new(), tokens: Vec::new()};
    
    *i += 1; // move to indent
    
    let macro_name: Token = tokens[*i].clone();

    if macro_name.token_type != TokenType::Ident {
        println!("MacroDecl: Expected identifier");
        exit(1);
    }

    for macron in macros.iter(){
        if macro_name.value == macron.name {
            println!("Macro already defined: #define {}", macro_name.value);
            exit(1);
        }
    }

    *i += 1; // move to terms

    macrom.name = macro_name.value;

    let mut terms: Vec<Token> = Vec::new();

    while tokens[*i].token_type != TokenType::NewLine {

        let token = tokens[*i].clone();

        if is_macro_token(token.token_type.clone()){
            println!("Cannot Put Macro inside #define");
            exit(1);
        }else{
            terms.push(token);
        }

        *i += 1;
    }

    macrom.tokens = terms;

    macros.push(macrom);
}

fn preprocess_ifdef_macro(i: &mut usize, tokens: &[Token], new_tokens: &mut Vec<Token>, macros: &mut Vec<Macro>){

    *i += 1;

    let macro_name: Token = tokens[*i].clone();

    if macro_name.token_type != TokenType::Ident {
        println!("Ifdef Macro: Expected identifier");
        exit(1);
    }

    *i += 1; // skip to terms

    let is_defined = {
        let mut answ: bool = false;
        for macrom in macros.iter(){
            if macro_name.value == macrom.name{
                answ = true;
                break;      
            }
        }

        answ
    };



    if is_defined{

        while tokens[*i].token_type != TokenType::ElseMacro && tokens[*i].token_type != TokenType::EndifMacro {
            let token: Token = tokens[*i].clone();

            if is_macro_token(token.token_type.clone()){
                preprocess_macro(i, tokens, new_tokens, macros);
            }else{
                new_tokens.push(token);
                *i += 1;
            }
        }

        if tokens[*i].token_type == TokenType::ElseMacro{
            while tokens[*i].token_type != TokenType::EndifMacro {
                *i+=1;
            }
        }

        *i+=1; // skip endif token
    }else{

        while tokens[*i].token_type != TokenType::ElseMacro && tokens[*i].token_type != TokenType::EndifMacro {
            *i+=1;
        }
        
        if tokens[*i].token_type == TokenType::ElseMacro{
            *i+=1;
            while tokens[*i].token_type != TokenType::EndifMacro {
                let token: Token = tokens[*i].clone();
        
                if is_macro_token(token.token_type.clone()){
                    preprocess_macro(i, tokens, new_tokens, macros);
                }else{
                    new_tokens.push(token);
                    *i += 1;
                }
            }
        }
        *i+=1; // skip endif token
    }

}
fn preprocess_ifndef_macro(i: &mut usize, tokens: &[Token], new_tokens: &mut Vec<Token>, macros: &mut Vec<Macro>){

    *i += 1;

    let macro_name: Token = tokens[*i].clone();

    if macro_name.token_type != TokenType::Ident {
        println!("Ifdef Macro: Expected identifier");
        exit(1);
    }

    *i += 1; // skip to terms

    let is_defined = {
        let mut answ: bool = false;
        for macrom in macros.iter(){
            if macro_name.value == macrom.name{
                answ = true;
                break;      
            }
        }

        answ
    };



    if !is_defined{

        while tokens[*i].token_type != TokenType::ElseMacro && tokens[*i].token_type != TokenType::EndifMacro {
            let token: Token = tokens[*i].clone();

            if is_macro_token(token.token_type.clone()){
                preprocess_macro(i, tokens, new_tokens, macros);
            }else{
                new_tokens.push(token);
                *i += 1;
            }
        }

        if tokens[*i].token_type == TokenType::ElseMacro{
            while tokens[*i].token_type != TokenType::EndifMacro {
                *i+=1;
            }
        }

        *i+=1; // skip endif token
    }else{

        while tokens[*i].token_type != TokenType::ElseMacro && tokens[*i].token_type != TokenType::EndifMacro {
            *i+=1;
        }
        
        if tokens[*i].token_type == TokenType::ElseMacro{
            *i+=1;
            while tokens[*i].token_type != TokenType::EndifMacro {
                let token: Token = tokens[*i].clone();
        
                if is_macro_token(token.token_type.clone()){
                    preprocess_macro(i, tokens, new_tokens, macros);
                }else{
                    new_tokens.push(token);
                    *i += 1;
                }
            }
        }
        *i+=1; // skip endif token
        

    }

}

fn preprocess_macro(i: &mut usize, tokens: &[Token], new_tokens: &mut Vec<Token>, macros: &mut Vec<Macro>){
    // let macrom: Macro = Macro{name: String::new(), tokens: Vec::new()};
    let token: Token = tokens[*i].clone();

    match token.token_type{

        TokenType::MacroDecl => {
            preprocess_macro_decl(i, tokens, macros);
        }
        
        TokenType::IfdefMacro => {
            preprocess_ifdef_macro(i, tokens, new_tokens, macros);
        }
        
        TokenType :: IfNdefMacro => {
            preprocess_ifndef_macro(i, tokens, new_tokens, macros);
        }

        TokenType:: EndifMacro => {
            println!("EndifMacro: You need to declare contition");
            exit(1);
        }

        TokenType :: ElseMacro => {
            println!("ElseMacro: You need to declare contition");
            exit(1);
        }

        _ => {
            println!("Unreachable, there are only macro tokens got {}", token.value);
            exit(1);
        }
    }

}

fn preprocess_macros(tokens: Vec<Token>) -> (Vec<Token>,Vec<Macro>){
    let mut new_tokens: Vec<Token> = Vec::new();
    let mut macros: Vec<Macro> = Vec::new();
    let mut i: usize = 0;
    let len: usize = tokens.len();

    while i<len{

        let token: Token = tokens[i].clone();

        if is_macro_token(token.token_type.clone()){
            preprocess_macro(&mut i, &tokens, &mut new_tokens, &mut macros)
        }else{
            new_tokens.push(token)
        }

        i+=1;
    }

    (new_tokens, macros)
}

pub fn preprocess_tokens(tokens: Vec<Token>) -> Vec<Token> {
    //TODO: Add ability to pass arguments into macros
    /*
    
        #include <stdlib.bf>

        #define car(color | type) write(color) `"\n"`. write(type) `"\n"`.

        car(`"red"` | `"toyota"`)

     */

    let (new_tokens,macros) = preprocess_macros(tokens);
    
    unwrap_macros(new_tokens, macros)
}
