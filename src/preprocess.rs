use crate::lex_file;
use std::fs;
use std::path::Path;
use std::process::exit;

// use std::process::exit;
use crate::common::{Size, Tape, Token, TokenType};

#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    pub tokens: Vec<Token>,
}

#[must_use]
pub fn is_macro(name: String, macros: &[Macro]) -> Option<Macro> {
    let mut macroms: Option<Macro> = None;
    for macrom in macros {
        if name == macrom.name {
            macroms = Some(macrom.clone());
            break;
        }
    }
    macroms
}

fn unwrap_macro(uno_macro: Macro, macros: &Vec<Macro>) -> Vec<Token> {
    let mut unwrap_token: Vec<Token> = Vec::new();
    for token in &uno_macro.tokens {
        if token.token_type != TokenType::Ident {
            unwrap_token.push(token.clone());
        } else {
            let name = token.value.clone();
            let nmacro = is_macro(name, macros);
            if let Some(nmacro) = nmacro {
                // if nmacro.is_some(){
                // let nmacro = nmacro.unwrap();
                let mut tokens = unwrap_macro(nmacro, macros);
                unwrap_token.append(&mut tokens);
            } else {
                println!("Unwrap Macros: Undentifier Not defined {}", token.value);
                exit(1);
            }
        }
    }
    unwrap_token
}

fn unwrap_macros(tokens: Vec<Token>, macros: Vec<Macro>) -> Vec<Token> {
    let mut next_tokens: Vec<Token> = Vec::new();
    let mut i: usize = 0;
    let len: usize = tokens.len();
    while i < len {
        let token = tokens[i].clone();

        if !(token.token_type == TokenType::Ident) {
            next_tokens.push(token);
        } else {
            let name = token.value.clone();
            let macrom = is_macro(name.clone(), &macros);
            if let Some(macrom) = macrom {
                let mut macro_tokens = unwrap_macro(macrom, &macros);
                next_tokens.append(&mut macro_tokens);
            } else {
                println!("Undentifier Not defined: {name}");
                exit(1);
            }
        }

        i += 1;
    }

    next_tokens
}

fn is_macro_token(token_type: TokenType) -> bool {
    token_type == TokenType::MacroDecl
        || token_type == TokenType::IfdefMacro
        || token_type == TokenType::IfNdefMacro
        || token_type == TokenType::ElseMacro
        || token_type == TokenType::EndifMacro
        || token_type == TokenType::TapeDecl
}

fn preprocess_macro_decl(i: &mut usize, tokens: &[Token], macros: &mut Vec<Macro>) {
    let mut macrom: Macro = Macro {
        name: String::new(),
        tokens: Vec::new(),
    };

    *i += 1; // move to indent

    let macro_name: Token = tokens[*i].clone();

    if macro_name.token_type != TokenType::Ident {
        println!("MacroDecl: Expected identifier");
        exit(1);
    }

    for macron in &*macros {
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

        if is_macro_token(token.token_type.clone()) {
            println!("Cannot Put Macro inside #define");
            exit(1);
        } else {
            terms.push(token);
        }

        *i += 1;
    }

    macrom.tokens = terms;

    macros.push(macrom);
}

fn preprocess_ifdef_macro(
    i: &mut usize,
    tokens: &[Token],
    new_tokens: &mut Vec<Token>,
    macros: &mut Vec<Macro>,
    tapes: &mut Vec<Tape>,
) {
    *i += 1;

    let macro_name: Token = tokens[*i].clone();

    if macro_name.token_type != TokenType::Ident {
        println!("Ifdef Macro: Expected identifier");
        exit(1);
    }

    *i += 1; // skip to terms

    let is_defined = {
        let mut answ: bool = false;
        for macrom in &*macros {
            if macro_name.value == macrom.name {
                answ = true;
                break;
            }
        }

        answ
    };

    if is_defined {
        while tokens[*i].token_type != TokenType::ElseMacro
            && tokens[*i].token_type != TokenType::EndifMacro
        {
            let token: Token = tokens[*i].clone();

            if is_macro_token(token.token_type.clone()) {
                preprocess_macro(i, tokens, new_tokens, macros, tapes);
            } else {
                new_tokens.push(token);
                *i += 1;
            }
        }

        if tokens[*i].token_type == TokenType::ElseMacro {
            while tokens[*i].token_type != TokenType::EndifMacro {
                *i += 1;
            }
        }

        *i += 1; // skip endif token
    } else {
        while tokens[*i].token_type != TokenType::ElseMacro
            && tokens[*i].token_type != TokenType::EndifMacro
        {
            *i += 1;
        }

        if tokens[*i].token_type == TokenType::ElseMacro {
            *i += 1;
            while tokens[*i].token_type != TokenType::EndifMacro {
                let token: Token = tokens[*i].clone();

                if is_macro_token(token.token_type.clone()) {
                    preprocess_macro(i, tokens, new_tokens, macros, tapes);
                } else {
                    new_tokens.push(token);
                    *i += 1;
                }
            }
        }
        *i += 1; // skip endif token
    }
}
fn preprocess_ifndef_macro(
    i: &mut usize,
    tokens: &[Token],
    new_tokens: &mut Vec<Token>,
    macros: &mut Vec<Macro>,
    tapes: &mut Vec<Tape>,
) {
    *i += 1;

    let macro_name: Token = tokens[*i].clone();

    if macro_name.token_type != TokenType::Ident {
        println!("Ifdef Macro: Expected identifier");
        exit(1);
    }

    *i += 1; // skip to terms

    let is_defined = {
        let mut answ: bool = false;
        for macrom in &*macros {
            if macro_name.value == macrom.name {
                answ = true;
                break;
            }
        }

        answ
    };

    if !is_defined {
        while tokens[*i].token_type != TokenType::ElseMacro
            && tokens[*i].token_type != TokenType::EndifMacro
        {
            let token: Token = tokens[*i].clone();

            if is_macro_token(token.token_type.clone()) {
                preprocess_macro(i, tokens, new_tokens, macros, tapes);
            } else {
                new_tokens.push(token);
                *i += 1;
            }
        }

        if tokens[*i].token_type == TokenType::ElseMacro {
            while tokens[*i].token_type != TokenType::EndifMacro {
                *i += 1;
            }
        }

        *i += 1; // skip endif token
    } else {
        while tokens[*i].token_type != TokenType::ElseMacro
            && tokens[*i].token_type != TokenType::EndifMacro
        {
            *i += 1;
        }

        if tokens[*i].token_type == TokenType::ElseMacro {
            *i += 1;
            while tokens[*i].token_type != TokenType::EndifMacro {
                let token: Token = tokens[*i].clone();

                if is_macro_token(token.token_type.clone()) {
                    preprocess_macro(i, tokens, new_tokens, macros, tapes);
                } else {
                    new_tokens.push(token);
                    *i += 1;
                }
            }
        }
        *i += 1; // skip endif token
    }
}

fn preprocess_tape_decl(i: &mut usize, tokens: &[Token], tapes: &mut Vec<Tape>) {
    let mut tape = Tape {
        name: String::new(),
        size: Size::Byte,
        cell_count: 0,
    };

    *i += 1;

    let name = tokens[*i].clone();

    if name.token_type != TokenType::Ident {
        println!("TapeDecl: Expected identifier");
        exit(1);
    }

    *i += 1;

    tape.name = name.value;

    let size = tokens[*i].clone();

    if size.token_type != TokenType::CellSize {
        println!(
            "TapeDecl({}): Expected CellSize (byte, word, dword, qword) got {}",
            tape.name, size.value
        );
        exit(1);
    }

    tape.size = match size.value.as_str() {
        "byte" => Size::Byte,

        "word" => Size::Word,

        "dword" => Size::Dword,

        "qword" => Size::Qword,

        _ => {
            println!("TapeDecl: Unreachable");
            exit(1);
        }
    };

    *i += 1;

    let cell_count = tokens[*i].clone();

    if cell_count.token_type != TokenType::IntLit {
        println!(
            "TapeDecl({}): Expected IntLit got {}",
            tape.name, size.value
        );
        exit(1);
    }

    tape.cell_count = cell_count.value.parse().unwrap();

    *i += 1;

    if tokens[*i].token_type != TokenType::NewLine {
        println!(
            "TapeDecl({}): Expected a new line got {}",
            tape.name, size.value
        );
        exit(1);
    }

    tapes.push(tape);
}

fn preprocess_macro(
    i: &mut usize,
    tokens: &[Token],
    new_tokens: &mut Vec<Token>,
    macros: &mut Vec<Macro>,
    tapes: &mut Vec<Tape>,
) {
    // let macrom: Macro = Macro{name: String::new(), tokens: Vec::new()};
    let token: Token = tokens[*i].clone();

    match token.token_type {
        TokenType::MacroDecl => {
            preprocess_macro_decl(i, tokens, macros);
        }

        TokenType::IfdefMacro => {
            preprocess_ifdef_macro(i, tokens, new_tokens, macros, tapes);
        }

        TokenType::IfNdefMacro => {
            preprocess_ifndef_macro(i, tokens, new_tokens, macros, tapes);
        }

        TokenType::EndifMacro => {
            println!("EndifMacro: You need to declare contition");
            exit(1);
        }

        TokenType::ElseMacro => {
            println!("ElseMacro: You need to declare contition");
            exit(1);
        }

        TokenType::TapeDecl => {
            preprocess_tape_decl(i, tokens, tapes);
        }

        _ => {
            println!(
                "Unreachable, there are only macro tokens got {}",
                token.value
            );
            exit(1);
        }
    }
}

fn preprocess_macros(tokens: Vec<Token>, tapes: &mut Vec<Tape>) -> (Vec<Token>, Vec<Macro>) {
    let mut new_tokens: Vec<Token> = Vec::new();
    let mut macros: Vec<Macro> = Vec::new();
    let mut i: usize = 0;
    let len: usize = tokens.len();

    while i < len {
        let token: Token = tokens[i].clone();

        if is_macro_token(token.token_type.clone()) {
            preprocess_macro(&mut i, &tokens, &mut new_tokens, &mut macros, tapes);
        } else {
            new_tokens.push(token);
        }

        i += 1;
    }

    (new_tokens, macros)
}

fn preprocess_include(
    i: &mut usize,
    tokens: &[Token],
    current_path: String,
    path: &String,
    new_tokens: &mut Vec<Token>,
    includes: &Vec<String>,
    _tapes: &mut Vec<Tape>,
) {
    let token = tokens[*i].clone();

    if token.token_type != TokenType::IncludeMacro {
        new_tokens.push(token);
        *i += 1;
    } else {
        *i += 1;
        let file_name: Token = tokens[*i].clone();

        if file_name.token_type != TokenType::StringLit
            && file_name.token_type != TokenType::IncludePath
        {
            println!("IncludeDecl: Expected String Literal or Include Path");
            exit(1);
        }

        *i += 1;

        let filename: String = {
            if file_name.token_type == TokenType::StringLit {
                path.clone() + file_name.value.as_str()
            } else {
                let include_path: Vec<&str> = file_name.value.split('/').collect::<Vec<&str>>();
                let len: usize = include_path.len() - 1;
                let mut exists: Option<String> = None;
                for path in includes {
                    let path_arr = path.split('/').collect::<Vec<&str>>();
                    let path_len = path_arr.len();
                    let mut path_str = {
                        let mut path = String::new();
                        for folder in path_arr.iter().take(path_len - len) {
                            path += folder;
                            path += "/";
                        }
                        path
                    };
                    path_str += file_name.value.as_str();
                    let path_path = Path::new(path_str.as_str());
                    if path_path.exists() {
                        exists = Some(path_str);
                        break;
                    }

                    dbg!(&path_str);
                }
                let out;
                if let Some(exists) = exists {
                    out = exists;
                } else {
                    println!("Could not find include: {}", file_name.value);
                    exit(1);
                }

                out
            }
        };

        // dbg!(&filename);

        // exit(1);

        if filename == current_path {
            println!("IncludeDecl: Cannot include file in itself file: {filename}");
            exit(1);
        }

        if !filename.ends_with(".bf") {
            println!("IncludeDecl: Brain fuck plus files must have .bf extension");
            exit(1);
        }

        let contents =
            fs::read_to_string(filename.clone()).expect("Something went wrong reading the file");

        let file_tokens = lex_file(contents);

        let mut file_i: usize = 0;
        let file_len: usize = file_tokens.len();

        while file_i < file_len {
            if token.token_type.clone() != TokenType::IncludeMacro {
                new_tokens.push(token.clone());
                file_i += 1;
            } else {
                preprocess_include(
                    &mut file_i,
                    &file_tokens,
                    filename.clone(),
                    path,
                    new_tokens,
                    includes,
                    _tapes,
                );
            }
        }

        // dbg!(file_tokens);

        // dbg!(new_tokens);

        // exit(1);
    }
}

fn include_includes(
    tokens: Vec<Token>,
    current_path: String,
    path: String,
    includes: Vec<String>,
    tapes: &mut Vec<Tape>,
) -> Vec<Token> {
    let mut i: usize = 0;
    let len: usize = tokens.len();
    let mut new_tokens: Vec<Token> = Vec::new();

    while i < len {
        preprocess_include(
            &mut i,
            &tokens,
            current_path.clone(),
            &path,
            &mut new_tokens,
            &includes,
            tapes,
        );
    }

    // dbg!(&new_tokens);

    new_tokens
}

pub fn preprocess_tokens(
    tokens: Vec<Token>,
    current_path: String,
    path: String,
    includes: Vec<String>,
    tapes: &mut Vec<Tape>,
) -> Vec<Token> {
    let new_tokens = include_includes(tokens, current_path, path, includes, tapes);

    let (new_tokens, macros) = preprocess_macros(new_tokens, tapes);

    unwrap_macros(new_tokens, macros)
}
