use std::fs;
use std::env;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::process::exit;

fn main() {
    // read the file contents into a string from args
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        exit(1);
    }

    let filename = &args[1];

    //check if extension is .bfp
    if !filename.ends_with(".bfp"){
        println!("Brain fuck plus files must have .bfp extension");
        exit(1);
    }

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");


    let mut file_content: String = String::new();
    file_content.push_str("BITS 64\n");
    file_content.push_str("section .text\n");
    file_content.push_str("global _start\n");
    file_content.push_str("_start:\n");

    let len = contents.len();

    #[derive(PartialEq, Debug)]
    struct Condition{
        addr: usize,
    }

    #[derive(PartialEq, Debug)]
    struct Forward{
        back_addr: usize
    }

    #[derive(PartialEq, Debug)]
    enum Jumps{
        Condition(Condition),
        Forward(Forward)
    }

    let mut jumps: Vec<Jumps> = Vec::new();

    //cross reference
    for i in 0..len{
        let ch = contents.chars().nth(i).unwrap();
        match ch{
            '['=>{
                jumps.push(Jumps::Condition(Condition { addr: 0 }));
            }
            ']'=>{
                let len = jumps.len();
                
                //find the last condition
                let mut last_condition = 0;
                for j in (0..len).rev(){
                    if jumps[j] == Jumps::Condition(Condition { addr: 0 }){
                        last_condition = j;
                        break;
                    }
                }

                //set the address of the condition
                match jumps[last_condition]{
                    Jumps::Condition(ref mut condition) => {
                        condition.addr = len;
                    }
                    _=>{}
                }

                //set the address of the forward to last condition
                jumps.push(Jumps::Forward(Forward { back_addr: last_condition }));
            }
            _=>{}
        }
    }

    let mut last_condition = 0;

    for i in 0..len{
        let ch = contents.chars().nth(i).unwrap();

        match ch{
            '>'=>{
                file_content.push_str("    add QWORD[pointer], 1\n");
            }
            '<'=>{
                file_content.push_str("    sub QWORD[pointer], 1\n");
            }
            '+'=>{
                file_content.push_str("    mov rax, mem\n");
                file_content.push_str("    add rax, QWORD[pointer]\n");
                file_content.push_str("    add BYTE [rax], 1\n");
            }
            '-'=>{
                file_content.push_str("    mov rax, mem\n");
                file_content.push_str("    add rax, QWORD[pointer]\n");
                file_content.push_str("    sub BYTE [rax], 1\n");
            }
            '.'=>{
                file_content.push_str("    mov rax, 1\n");
                file_content.push_str("    mov rdi, 1\n");
                file_content.push_str("    mov rsi, mem\n");
                file_content.push_str("    add rsi, QWORD[pointer]\n");
                file_content.push_str("    mov rdx, 1\n");
                file_content.push_str("    syscall\n");
            }
            ','=>{
                file_content.push_str("    mov rax, 0\n");
                file_content.push_str("    mov rdi, 0\n");
                file_content.push_str("    mov rsi, mem\n");
                file_content.push_str("    add rsi, QWORD[pointer]\n");
                file_content.push_str("    mov rdx, 1\n");
                file_content.push_str("    syscall\n");
            }
            '['=>{
                file_content.push_str("    mov rax, mem\n");
                file_content.push_str("    add rax, QWORD[pointer]\n");
                file_content.push_str("    mov al, byte[rax]\n");
                file_content.push_str("    cmp al, 0\n");

                let mut condition_id = 0;
                for j in last_condition..len{
                    if matches!(jumps[j], Jumps::Condition(_)){
                        condition_id = j;
                        break;
                    }
                }

                last_condition = condition_id+1;

                let condition = match jumps[condition_id]{
                    Jumps::Condition(ref condition) => {
                        condition
                    }
                    _=>{panic!("condition not found")}
                };

                let forward_id = condition.addr;

                file_content.push_str(format!("    je forward_{}\n",forward_id).as_str());
                file_content.push_str(format!("condition_{}:\n",condition_id).as_str());

            }
            ']'=>{
                file_content.push_str("    mov rax, mem\n");
                file_content.push_str("    add rax, QWORD[pointer]\n");
                file_content.push_str("    mov al, byte[rax]\n");
                file_content.push_str("    cmp al, 0\n");

                let mut forward_id = 0;
                for j in last_condition..len{
                    if matches!(jumps[j], Jumps::Forward(_)){
                        forward_id = j;
                        break;
                    }
                }

                last_condition = forward_id+1;

                let forward = match jumps[forward_id]{
                    Jumps::Forward(ref forward) => {
                        forward
                    }
                    _=>{panic!("forward not found")}
                };

                let condition_id = forward.back_addr;

                file_content.push_str(format!("    jne condition_{}\n",condition_id).as_str());
                file_content.push_str(format!("forward_{}:\n",forward_id).as_str());


            }
            _=>{}
        }

    }

    file_content.push_str("    mov rax, 60\n");
    file_content.push_str("    mov rdi, 0\n");
    file_content.push_str("    syscall\n");

    file_content.push_str("section .bss\n");
    file_content.push_str("    pointer: resb 8\n");
    file_content.push_str(format!("    mem: resb {} \n", 1024*1024).as_str());


    // create a new file
    let path = format!("{}", filename.replace(".bfp", ".asm"));
    let path = Path::new(&path);
    let display = path.display();

    let mut file = match File::create(&path){
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(file_content.as_bytes()){
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => println!("Successfully wrote to {}", display),
    }

    std::process::Command::new("nasm")
        .arg("-felf64")
        .arg("-g")
        .arg(&format!("{}", filename.replace(".bfp", ".asm")))
        .output()
        .expect("failed to execute process");

    // use ld to link
    std::process::Command::new("ld")
        .arg("-o")
        .arg(&format!("{}", filename.replace(".bfp", "")))
        .arg(&format!("{}", filename.replace(".bfp", ".o")))
        .output()
        .expect("failed to execute process");

    println!("Generated executable");
}