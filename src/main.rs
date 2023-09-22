use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;

pub mod common;
use common::*;

pub mod lexer;
use lexer::lex_file;

pub mod code_gen;
use crate::code_gen::generate_code;
use crate::parser::parse_file;

pub mod preprocess;
use preprocess::*;

pub mod parser;

fn main() {
    // read the file contents into a string from args
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        exit(1);
    }

    let filename = &args[1];

    //check if extension is .bf
    if !filename.ends_with(".bf") {
        println!("Brain fuck plus files must have .bf extension");
        exit(1);
    }

    let contents = fs::read_to_string(filename.clone()).expect("Something went wrong reading the file");

    let path: String = {
        let mut temp = String::new();
        let arr = filename.split('/').collect::<Vec<&str>>();
        let len = arr.len();

        for folder in arr.iter().take(len-1){
            temp += folder;
            temp += "/";
        }

        temp
    };

    // dbg!(path);

    // exit(1);

    let mut file_content: String = String::new();

    //Boilerplate
    file_content.push_str("BITS 64\n");
    file_content.push_str("section .text\n");
    file_content.push_str("global _start\n");
    file_content.push_str("_start:\n");

    let tokens = lex_file(contents);
    // dbg!(&tokens);
    let tokens = preprocess_tokens(tokens, filename.clone(),path);
    let operations = parse_file(tokens);

    generate_code(operations, &mut file_content);

    // generate_code_backup(tokens, &mut file_content);

    // Rest of boilerplate
    file_content.push_str("    mov rax, 60\n");
    file_content.push_str("    mov rdi, 0\n");
    file_content.push_str("    syscall\n");

    file_content.push_str("section .bss\n");
    file_content.push_str("    pointer: resb 8\n");
    file_content.push_str(format!("    mem: resb {} \n", MEM_SIZE).as_str());

    // create a new file
    let path = filename.replace(".bf", ".asm");
    let path = Path::new(&path);
    let display = path.display();

    let mut file = match File::create(path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(file_content.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => println!("Successfully wrote to {}", display),
    }

    std::process::Command::new("nasm")
        .arg("-felf64")
        .arg("-g")
        .arg(&filename.replace(".bf", ".asm"))
        .output()
        .expect("failed to execute process");

    // use ld to link
    std::process::Command::new("ld")
        .arg("-o")
        .arg(&filename.replace(".bf", ""))
        .arg(&filename.replace(".bf", ".o"))
        .output()
        .expect("failed to execute process");

    #[cfg(not(debug_assertions))]
    std::process::Command::new("rm")
        .arg(&filename.replace(".bf", ".asm"))
        .output()
        .expect("failed to execute process");

    std::process::Command::new("rm")
        .arg(&filename.replace(".bf", ".o"))
        .output()
        .expect("failed to execute process");

    println!("Generated executable");
}
