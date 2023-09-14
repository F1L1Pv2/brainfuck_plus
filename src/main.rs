use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;


#[derive(Debug)]
enum TokenType {
    Word,
    PointerLeft,
    PointerRight,
    PointerBack,
    Add,
    Sub,
    ReadByte,
    WriteByte,
    MemAddr,
    BaseMemAddr,
    Syscall,
    Clear,
    ZeroJump,
    NonZeroJump,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: String,
}

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

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut file_content: String = String::new();
    file_content.push_str("BITS 64\n");
    file_content.push_str("section .text\n");
    file_content.push_str("global _start\n");
    file_content.push_str("_start:\n");

    let len = contents.len();

    #[derive(PartialEq, Debug)]
    struct Condition {
        addr: usize,
    }

    #[derive(PartialEq, Debug)]
    struct Forward {
        back_addr: usize,
    }

    #[derive(PartialEq, Debug)]
    enum Jumps {
        Condition(Condition),
        Forward(Forward),
    }

    let mut jumps: Vec<Jumps> = Vec::new();

    
    //cross reference
    for i in 0..len {
        let ch = contents.chars().nth(i).unwrap();
        match ch {
            '[' => {
                jumps.push(Jumps::Condition(Condition { addr: 0 }));
            }
            ']' => {
                let len = jumps.len();

                //find the last condition
                let mut last_condition = 0;
                for j in (0..len).rev() {
                    if jumps[j] == Jumps::Condition(Condition { addr: 0 }) {
                        last_condition = j;
                        break;
                    }
                }

                //set the address of the condition
                match jumps[last_condition] {
                    Jumps::Condition(ref mut condition) => {
                        condition.addr = len;
                    }
                    _ => {}
                }

                //set the address of the forward to last condition
                jumps.push(Jumps::Forward(Forward {
                    back_addr: last_condition,
                }));
            }
            _ => {}
        }
    }

    let mut last_condition = 0;

    let mem_size = 1024 * 1024;
    let mut mem_dbg_ln = 0;

    let mut i: usize = 0;
    
    let mut tokens: Vec<Token> = Vec::new();
    let mut comment = false;

    while i < len {
        let ch = contents.chars().nth(i).unwrap();

        if !comment{

        match ch {
            '>' => {
                tokens.push(Token {
                    token_type: TokenType::PointerRight,
                    value: ch.to_string(),
                });
            }
            '<' => {
                tokens.push(Token {
                    token_type: TokenType::PointerLeft,
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
            '.' => {
                tokens.push(Token {
                    token_type: TokenType::WriteByte,
                    value: ch.to_string(),
                });
            }
            ',' => {
                tokens.push(Token {
                    token_type: TokenType::ReadByte,
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
            '$' => {
                tokens.push(Token {
                    token_type: TokenType::MemAddr,
                    value: ch.to_string(),
                });
            }
            '%' => {
                tokens.push(Token {
                    token_type: TokenType::BaseMemAddr,
                    value: ch.to_string(),
                });
            }
            '\'' => {
                tokens.push(Token {
                    token_type: TokenType::Clear,
                    value: ch.to_string(),
                });
            }
            '?' => {
                tokens.push(Token {
                    token_type: TokenType::Syscall,
                    value: ch.to_string(),
                });
            }
            '&' => {
                tokens.push(Token {
                    token_type: TokenType::PointerBack,
                    value: ch.to_string(),
                });
            }

            _ => {

                if ch.is_whitespace(){
                    i+=1;
                    continue;
                }

                let next_char =  contents.chars().nth(i+1).unwrap();
                let checker = ch.to_string() + next_char.to_string().as_str();

                if checker == "/*"{
                    comment = true;
                    i+=2;
                }else{
                    i+=1;
                }
                continue;
            }
        }

        i += 1;

        }else{

            let next_char =  contents.chars().nth(i+1).unwrap();

            let checker = ch.to_string() + next_char.to_string().as_str();

            if checker == "*/"{
                comment = false;
                i+=2;
            }else{
                i+=1;
            }
        }
    }

    // dbg!(&tokens);

    // exit(1);

    // for i in 0..len {
    for token in tokens.iter(){
        // let ch = contents.chars().nth(i).unwrap();

        // match ch {
        match token.token_type{
            // '>' => {
            TokenType::PointerRight => {
                // file_content.push_str("    add QWORD[pointer], 1\n");

                //check if pointer is at the end
                file_content
                    .push_str(format!("    cmp QWORD[pointer], {}\n", mem_size - 1).as_str());
                file_content.push_str(format!("    je bound_{}\n", i).as_str());
                file_content.push_str("    add QWORD[pointer], 1\n");
                file_content.push_str(format!("    jmp skip_{}\n", i).as_str());
                file_content.push_str(format!("bound_{}:\n", i).as_str());
                file_content.push_str("    mov QWORD[pointer], 0\n");
                file_content.push_str(format!("skip_{}:\n", i).as_str());
            }
            // '<' => {
            TokenType::PointerLeft => {
                //check if pointer is zero
                file_content.push_str("    cmp QWORD[pointer], 0\n");
                file_content.push_str(format!("    je bound_{}\n", i).as_str());
                file_content.push_str("    sub QWORD[pointer], 1\n");
                file_content.push_str(format!("    jmp skip_{}\n", i).as_str());
                file_content.push_str(format!("bound_{}:\n", i).as_str());
                file_content
                    .push_str(format!("    mov QWORD[pointer], {}\n", mem_size - 1).as_str());
                file_content.push_str(format!("skip_{}:\n", i).as_str());

                // file_content.push_str("    sub QWORD[pointer], 1\n");
            }
            // '$' => {
            TokenType::MemAddr =>{
                // put current mem addr into cell
                file_content.push_str(format!("debug_mem_{}:\n", mem_dbg_ln).as_str());
                file_content.push_str("    mov rax, mem\n");
                file_content.push_str("    add rax, QWORD[pointer]\n");
                file_content.push_str("    mov rbx, rax\n");
                file_content.push_str("    mov QWORD[rax], rbx\n");
                mem_dbg_ln += 1;
            }

            // '%' => {
            TokenType::BaseMemAddr =>{
                // put base mem addr into cell
                file_content.push_str("    mov rax, mem\n");
                file_content.push_str("    add rax, QWORD[pointer]\n");
                file_content.push_str("    mov rbx, mem\n");
                file_content.push_str("    mov QWORD[rax], rbx\n");
            }

            // '&' => {
            TokenType::PointerBack => {
                // set pointer to 0
                file_content.push_str("    mov QWORD[pointer], 0\n");
            }

            // '?' => {
            TokenType::Syscall => {
                // perform syscall
                file_content.push_str("    mov rbp, mem\n");
                file_content.push_str("    add rbp, QWORD[pointer]\n");
                file_content.push_str("    mov rax, QWORD[rbp]\n");
                file_content.push_str("    add rbp, 8\n");
                file_content.push_str("    mov rdi, QWORD[rbp]\n");
                file_content.push_str("    add rbp, 8\n");
                file_content.push_str("    mov rsi, QWORD[rbp]\n");
                file_content.push_str("    add rbp, 8\n");
                file_content.push_str("    mov rdx, QWORD[rbp]\n");
                file_content.push_str("    syscall\n");
            }

            // '\'' => {
            TokenType::Clear => {
                // clear current cell
                file_content.push_str("   mov rax, mem\n");
                file_content.push_str("   add rax, QWORD[pointer]\n");
                file_content.push_str("   mov BYTE[rax], 0\n");
            }

            // '+' => {
            TokenType::Add => {
                file_content.push_str("    mov rax, mem\n");
                file_content.push_str("    add rax, QWORD[pointer]\n");
                file_content.push_str("    add BYTE [rax], 1\n");
            }
            // '-' => {
            TokenType::Sub => {
                file_content.push_str("    mov rax, mem\n");
                file_content.push_str("    add rax, QWORD[pointer]\n");
                file_content.push_str("    sub BYTE [rax], 1\n");
            }
            // '.' => {
            TokenType::WriteByte => {
                file_content.push_str("    mov rax, 1\n");
                file_content.push_str("    mov rdi, 1\n");
                file_content.push_str("    mov rsi, mem\n");
                file_content.push_str("    add rsi, QWORD[pointer]\n");
                file_content.push_str("    mov rdx, 1\n");
                file_content.push_str("    syscall\n");
            }
            // ',' => {
            TokenType::ReadByte => {
                file_content.push_str("    mov rax, 0\n");
                file_content.push_str("    mov rdi, 0\n");
                file_content.push_str("    mov rsi, mem\n");
                file_content.push_str("    add rsi, QWORD[pointer]\n");
                file_content.push_str("    mov rdx, 1\n");
                file_content.push_str("    syscall\n");
            }
            // '[' => {
            TokenType::ZeroJump => {
                file_content.push_str("    mov rax, mem\n");
                file_content.push_str("    add rax, QWORD[pointer]\n");
                file_content.push_str("    mov al, byte[rax]\n");
                file_content.push_str("    cmp al, 0\n");

                let mut condition_id = 0;
                for j in last_condition..len {
                    if matches!(jumps[j], Jumps::Condition(_)) {
                        condition_id = j;
                        break;
                    }
                }

                last_condition = condition_id + 1;

                let condition = match jumps[condition_id] {
                    Jumps::Condition(ref condition) => condition,
                    _ => {
                        panic!("condition not found")
                    }
                };

                let forward_id = condition.addr;

                file_content.push_str(format!("    je forward_{}\n", forward_id).as_str());
                file_content.push_str(format!("condition_{}:\n", condition_id).as_str());
            }
            // ']' => {
            TokenType::NonZeroJump => {
                file_content.push_str("    mov rax, mem\n");
                file_content.push_str("    add rax, QWORD[pointer]\n");
                file_content.push_str("    mov al, byte[rax]\n");
                file_content.push_str("    cmp al, 0\n");

                let mut forward_id = 0;
                for j in last_condition..len {
                    if matches!(jumps[j], Jumps::Forward(_)) {
                        forward_id = j;
                        break;
                    }
                }

                last_condition = forward_id + 1;

                let forward = match jumps[forward_id] {
                    Jumps::Forward(ref forward) => forward,
                    _ => {
                        panic!("forward not found")
                    }
                };

                let condition_id = forward.back_addr;

                file_content.push_str(format!("    jne condition_{}\n", condition_id).as_str());
                file_content.push_str(format!("forward_{}:\n", forward_id).as_str());
            }
            _ => {}
        }
    }

    file_content.push_str("    mov rax, 60\n");
    file_content.push_str("    mov rdi, 0\n");
    file_content.push_str("    syscall\n");

    file_content.push_str("section .bss\n");
    file_content.push_str("    pointer: resb 8\n");
    file_content.push_str(format!("    mem: resb {} \n", mem_size).as_str());

    // create a new file
    let path = format!("{}", filename.replace(".bf", ".asm"));
    let path = Path::new(&path);
    let display = path.display();

    let mut file = match File::create(&path) {
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
        .arg(&format!("{}", filename.replace(".bf", ".asm")))
        .output()
        .expect("failed to execute process");

    // use ld to link
    std::process::Command::new("ld")
        .arg("-o")
        .arg(&format!("{}", filename.replace(".bf", "")))
        .arg(&format!("{}", filename.replace(".bf", ".o")))
        .output()
        .expect("failed to execute process");

    println!("Generated executable");
}
