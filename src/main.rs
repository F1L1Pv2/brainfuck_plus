use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;

pub mod common;
use common::{MEM_SIZE, Size, Tape};

pub mod lexer;
use lexer::lex_file;

pub mod code_gen;
use crate::code_gen::generate_code;
use crate::parser::parse_file;

pub mod preprocess;
use preprocess::preprocess_tokens;

pub mod parser;

fn usage(filename: String) {

    let mut arr = filename.split('/').collect::<Vec<&str>>();
    arr.reverse();

    println!("USAGE: {} <options> <filename>", arr[0]);
    println!("-i | -I include folder path (add only one)");
    println!("-l | -L libs folder path (add only one)");
    println!("-o | -O out file name (if not provided bf+ will use name of file)");
}

fn main() {
    // read the file contents into a string from args
    let args: Vec<String> = env::args().collect();

    let mut arg_i: usize = 1;
    let argc: usize = args.len();
    let mut filename: String = String::new();
    let mut out_file_path: String = String::new();

    let mut includes: Vec<String> = Vec::new();
    let mut libs: Vec<String> = Vec::new();

    let std_lib_path = {
        let arr = args[0].split('/').collect::<Vec<&str>>();
        let len = arr.len() - 1;
        let mut out = String::new();

        for folder in arr.iter().take(len){
            out += folder;
            out += "/";
        }

        out
    };

    includes.push(std_lib_path);

    // dbg!(std_lib_path);

    // exit(1);

    // if args.len() < 2 {
    //     println!("USAGE: {} <filename>", args[0]);
    //     exit(1);
    // }

    while arg_i < argc{

        let arg = args[arg_i].clone();

        match arg.as_str(){

            "-I" => {
                arg_i += 1;
                if arg_i < argc{
                    includes.push(args[arg_i].clone())
                }
            }

            "-i" => {
                arg_i += 1;
                if arg_i < argc{
                    includes.push(args[arg_i].clone())
                }
            }

            "-l" => {
                arg_i += 1;
                if arg_i < argc{
                    libs.push(args[arg_i].clone())
                }
            }

            "-L" => {
                arg_i += 1;
                if arg_i < argc{
                    libs.push(args[arg_i].clone())
                }
            }

            "-o" => {
                arg_i += 1;
                if arg_i < argc{
                    out_file_path = args[arg_i].clone();
                }
            }

            "-O" => {
                arg_i += 1;
                if arg_i < argc{
                    out_file_path = args[arg_i].clone();
                }
            }

            _ => {
                filename = arg.clone();
            }
        }


        arg_i+=1;
    }


    if !libs.is_empty(){
        usage(args[0].clone());
        println!("Libs are not currently implemented");
        exit(1);
    }



    if filename == String::new() {
        usage(args[0].clone());
        println!("Filename Wasn't provided");
        exit(1);
    }

    if out_file_path == String::new() {
        out_file_path = filename.replace(".bf", "");
    }

    //check if extension is .bf
    if !filename.ends_with(".bf") {
        println!("Brain fuck plus files must have .bf extension");
        exit(1);
    }

    let contents = fs
        ::read_to_string(filename.clone())
        .expect("Something went wrong reading the file");

    let path: String = {
        let mut temp = String::new();
        let arr = filename.split('/').collect::<Vec<&str>>();
        let len = arr.len();

        for folder in arr.iter().take(len - 1) {
            temp += folder;
            temp += "/";
        }

        temp
    };

    // dbg!(path);

    let mut file_content: String = String::new();

    let mut tapes: Vec<Tape> = vec![Tape{name: "main".to_string(), size: Size::Byte, cell_count: MEM_SIZE}];

    //Boilerplate
    file_content.push_str("BITS 64\n");
    file_content.push_str("section .text\n");
    file_content.push_str("global _start\n");
    file_content.push_str("_start:\n");

    let tokens = lex_file(contents);
    // dbg!(&tokens);
    let tokens = preprocess_tokens(tokens, filename.clone(), path, includes, &mut tapes);
    let operations = parse_file(tokens, &tapes);

    generate_code(operations, &mut file_content, &tapes);

    // generate_code_backup(tokens, &mut file_content);

    // Rest of boilerplate
    file_content.push_str("    mov rax, 60\n");
    file_content.push_str("    mov rdi, 0\n");
    file_content.push_str("    syscall\n");

    file_content.push_str("section .bss\n");
    // file_content.push_str("    pointer: resb 8\n");
    // file_content.push_str(format!("    mem: resb {} \n", MEM_SIZE).as_str());

    file_content.push_str("; -------- tapes -------- ; \n");
    
    for tape in &tapes{

        let size_str = match tape.size{
            Size::Byte => {
                "resb"
            }
            Size::Word => {
                "resw"
            }
            Size::Dword => {
                "resd"
            }
            Size::Qword => {
                "resq"
            }
        };

        file_content.push_str(format!("    {}_pointer: resq 1\n",tape.name).as_str());
        file_content.push_str(format!("    {}: {} {}\n",tape.name,size_str,tape.cell_count).as_str());
    }


    // dbg!(file_content);
    // print!("{}",file_content);

    // exit(1);

    // create a new file
    let path = filename.replace(".bf", ".asm");
    let path = Path::new(&path);
    let display = path.display();

    let mut file = match File::create(path) {
        Err(why) => panic!("Couldn't create {display}: {why}"),
        Ok(file) => file,
    };

    match file.write_all(file_content.as_bytes()) {
        Err(why) => panic!("Couldn't write to {display}: {why}"),
        Ok(_) => println!("Successfully wrote to {display}"),
    }

    std::process::Command
        ::new("nasm")
        .arg("-felf64")
        .arg("-g")
        .arg(&filename.replace(".bf", ".asm"))
        .output()
        .expect("failed to execute process");

    // use ld to link
    std::process::Command
        ::new("ld")
        .arg("-o")
        .arg(&out_file_path)
        .arg(&filename.replace(".bf", ".o"))
        .output()
        .expect("failed to execute process");

    #[cfg(not(debug_assertions))]
    std::process::Command
        ::new("rm")
        .arg(&filename.replace(".bf", ".asm"))
        .output()
        .expect("failed to execute process");

    std::process::Command
        ::new("rm")
        .arg(&filename.replace(".bf", ".o"))
        .output()
        .expect("failed to execute process");

    println!("Generated executable");
}
