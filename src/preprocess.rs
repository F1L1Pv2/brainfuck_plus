use crate::lex_file;
use std::fs;
use std::path::Path;
use std::process::exit;

// use std::process::exit;
use crate::common::*;

#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    pub tokens: Vec<Token>,
    pub args: Vec<String>,
}

pub fn is_macro(name: String, macros: &[Macro]) -> Option<Macro> {
    let mut macroms: Option<Macro> = None;
    for macrom in macros.iter() {
        if name == macrom.name {
            macroms = Some(macrom.clone());
            break;
        }
    }
    macroms
}

fn exec_macro(
    i: &mut usize,
    uno_macro: &Macro,
    tokens: &[Token],
    next_tokens: &mut Vec<Token>,
    macros: &Vec<Macro>,
    pass_args: &Vec<Vec<Token>>,
) {
    // dbg!(&uno_macro);

    let token = tokens[*i].clone();

    if token.token_type != TokenType::Ident {
        next_tokens.push(token);
    } else {
        let mut is_arg: bool = false;
        let mut arg_num: usize = 0;

        for (n, arg) in uno_macro.args.iter().enumerate() {
            if &token.value == arg {
                is_arg = true;
                arg_num = n;
                break;
            }
        }

        if is_arg {
            // dbg!(pass_args);
            // dbg!(&token);

            let arg_token = pass_args[arg_num].clone();

            // dbg!(&arg_token);

            let mut n: usize = 0;

            let base = arg_token[n].clone();

            if base.token_type != TokenType::Ident {
                next_tokens.push(base);
            } else {
                let name = base.value.clone();
                let macrom = is_macro(name.clone(), macros);
                if let Some(macrom) = macrom {
                    let mut local_pass_args: Vec<Vec<Token>> = Vec::new();

                    if arg_token.len() > 1 {
                        n += 1;

                        let open_paren = arg_token[n].clone();

                        if open_paren.token_type == TokenType::OpenParen {
                            n += 1;

                            let mut arg: Vec<Token> = Vec::new();

                            let mut closure: usize = 1;

                            // while tokens[*i].token_type != TokenType::CloseParen {
                            while closure > 0 {
                                let token = arg_token[n].clone();

                                if token.token_type == TokenType::OpenParen {
                                    closure += 1;
                                    arg.push(token);
                                    n += 1;
                                    continue;
                                }

                                if token.token_type == TokenType::CloseParen {
                                    closure -= 1;
                                    arg.push(token);
                                    n += 1;
                                    continue;
                                }

                                if token.token_type == TokenType::SplitArg {
                                    local_pass_args.push(arg.clone());
                                    dbg!(&arg);
                                    arg = Vec::new();
                                } else {
                                    arg.push(token);
                                }

                                n += 1;
                            }

                            arg.pop();

                            local_pass_args.push(arg.clone());

                            // n += 1;
                        }
                        //  else {
                        //     n -= 1;
                        // }
                    }

                    // println!("Local pass args");
                    // dbg!(&local_pass_args);

                    // exit(1);

                    // dbg!(&local_pass_args);

                    // exit(1);

                    let mut macro_tokens = unwrap_macro(macrom, macros, &local_pass_args);
                    next_tokens.append(&mut macro_tokens);
                } else {
                    println!("Undentifier Not defined: {}", name);
                    exit(1);
                }

                // exec_macro(i, uno_macro, tokens, next_tokens, macros, local_pass_args);
            }

            // dbg!(&arg_token);

            // exit(1);

            return;
        }

        let name = token.value.clone();
        let macrom = is_macro(name.clone(), macros);
        if let Some(macrom) = macrom {
            let mut local_pass_args: Vec<Vec<Token>> = Vec::new();

            if tokens.len() > 1  && *i + 1 < tokens.len(){

                dbg!(&tokens);

                *i += 1;

                let open_paren = tokens[*i].clone();

                if open_paren.token_type == TokenType::OpenParen {
                    *i += 1;

                    let mut arg: Vec<Token> = Vec::new();

                    // while tokens[*i].token_type != TokenType::CloseParen {
                    let mut closure: usize = 1;

                    // while tokens[*i].token_type != TokenType::CloseParen {
                    while closure > 0 {
                        // println!("{:?}",&tokens[*i].value);

                        let token = tokens[*i].clone();

                        if token.token_type == TokenType::OpenParen {
                            closure += 1;
                            arg.push(token);
                            *i += 1;
                            continue;
                        }

                        if token.token_type == TokenType::CloseParen {
                            closure -= 1;
                            arg.push(token);
                            *i += 1;
                            continue;
                        }

                        if token.token_type == TokenType::SplitArg {
                            local_pass_args.push(arg.clone());
                            // dbg!(&arg);
                            arg = Vec::new();
                        } else {
                            arg.push(token);
                        }

                        *i += 1;
                    }

                    arg.pop();

                    local_pass_args.push(arg.clone());
                } else {
                    *i -= 1;
                }
            }

            // println!("Local pass args");
            // dbg!(&local_pass_args);

            // exit(1);

            let mut macro_tokens = unwrap_macro(macrom, macros, &local_pass_args);
            next_tokens.append(&mut macro_tokens);
        } else {
            println!("Undentifier Not defined: {}", name);
            exit(1);
        }
    }
}

fn unwrap_macro(uno_macro: Macro, macros: &Vec<Macro>, pass_args: &Vec<Vec<Token>>) -> Vec<Token> {
    let mut unwrap_tokens: Vec<Token> = Vec::new();
    let mut i: usize = 0;
    let len: usize = uno_macro.tokens.len();
    while i < len {
        exec_macro(
            &mut i,
            &uno_macro,
            &uno_macro.tokens,
            &mut unwrap_tokens,
            macros,
            pass_args,
        );

        i += 1;
    }
    unwrap_tokens
}

fn unwrap_macros(tokens: Vec<Token>, macros: Vec<Macro>) -> Vec<Token> {
    let mut next_tokens: Vec<Token> = Vec::new();
    let mut i: usize = 0;
    let len: usize = tokens.len();
    while i < len {
        // let token = tokens[i].clone();
        exec_macro(
            &mut i,
            &Macro {
                name: String::new(),
                tokens: Vec::new(),
                args: Vec::new(),
            },
            &tokens,
            &mut next_tokens,
            &macros,
            &Vec::new(),
        );

        // dbg!(&tokens[i]);
        if tokens[i].token_type != TokenType::NewLine {
            next_tokens.push(tokens[i].clone());
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
        args: Vec::new(),
    };

    *i += 1; // move to indent

    let macro_name: Token = tokens[*i].clone();

    if macro_name.token_type != TokenType::Ident {
        println!("MacroDecl: Expected identifier");
        exit(1);
    }

    for macron in macros.iter() {
        if macro_name.value == macron.name {
            println!("Macro already defined: #define {}", macro_name.value);
            exit(1);
        }
    }

    *i += 1; // move to terms

    // dbg!(&tokens[*i]);

    // exit(1);
    let open_paren = tokens[*i].clone();

    if open_paren.token_type == TokenType::OpenParen {
        *i += 1;
        let mut split = false;
        while tokens[*i].token_type != TokenType::CloseParen {
            let token = tokens[*i].clone();

            if !split {
                if token.token_type != TokenType::Ident {
                    println!("MacroDecl: Expected identifier inside parameter decl");
                    exit(1);
                }
                macrom.args.push(token.value);
                split = true
            } else {
                if token.token_type != TokenType::SplitArg {
                    println!("MacroDecl: Expected args splitter");
                    exit(1);
                }

                split = false;
            }

            *i += 1;
        }
        *i += 1;
    }

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
        for macrom in macros.iter() {
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
        for macrom in macros.iter() {
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

    *i += 1;
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
            // i += 1;
        }

        i += 1;
    }

    (new_tokens, macros)
}

fn preprocess_include(
    i: &mut usize,
    tokens: &Vec<Token>,
    current_path: String,
    path: &String,
    new_tokens: &mut Vec<Token>,
    includes: &Vec<String>,
    tapes: &mut Vec<Tape>,
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
                let include_path: Vec<&str> = file_name.value.split("/").collect::<Vec<&str>>();
                let len: usize = include_path.len() - 1;
                let mut exists: Option<String> = None;
                for path in includes.iter() {
                    let path_arr = path.split("/").collect::<Vec<&str>>();
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
            println!(
                "IncludeDecl: Cannot include file in itself file: {}",
                filename
            );
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
                    tapes,
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
    //TODO: Add ability to pass arguments into macros
    /*

       #include <stdlib.bf>

       #define car(color | type) write(color) `"\n"`. write(type) `"\n"`.

       car(`"red"` | `"toyota"`)

    */

    let new_tokens = include_includes(tokens, current_path, path, includes, tapes);

    let (new_tokens, macros) = preprocess_macros(new_tokens, tapes);

    // dbg!(&macros);

    // let out = unwrap_macros(new_tokens, macros);
    // dbg!(&out);
    // out
    unwrap_macros(new_tokens, macros)
}
