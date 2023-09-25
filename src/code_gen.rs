use crate::common::*;
use std::process::exit;

fn cross_reference(operations: &[Operation]) -> Vec<Jumps> {
    let mut jumps: Vec<Jumps> = Vec::new();

    for operation in operations.iter() {
        match operation.token_type {
            TokenType::ZeroJump => {
                jumps.push(Jumps::Condition(Condition { addr: 0 }));
            }
            TokenType::NonZeroJump => {
                let len = jumps.len();

                //find the last condition
                let mut last_condition = 0;
                for j in (0..len).rev() {
                    if jumps[j] == Jumps::Condition(Condition { addr: 0 }) {
                        last_condition = j;
                        break;
                    }
                }

                if let Jumps::Condition(ref mut condition) = jumps[last_condition] {
                    condition.addr = len;
                }

                //set the address of the forward to last condition
                jumps.push(Jumps::Forward(Forward {
                    back_addr: last_condition,
                }));
            }
            _ => {}
        }
    }

    jumps
}

pub fn generate_code(operations: Vec<Operation>, file_content: &mut String, tapes: &Vec<Tape>) {
    let mut last_condition = 0;
    // let mut mem_dbg_ln = 0;

    let len = operations.len();
    
    let jumps = cross_reference(&operations);
    // let jumps = cross_reference(&operations);

    let mut current_tape = 0;
    
    for (i,operation) in operations.iter().enumerate() {
        // for i in 0..len {
        //     let ch = contents.chars().nth(i).unwrap();
                
        let tape: Tape = if let Some(tape_val) = operation.tape.clone(){
            tape_val
        }else{
            tapes[current_tape].clone()
        };

        let cell_size: usize = match tape.size{
            Size::Byte =>{
                1
            }

            Size::Word=>{
                2
            }

            Size::Dword=>{
                4
            }

            Size::Qword=>{
                8
            }
        };

        let cell_size_str: &str = match tape.size{
            Size::Byte =>{
                "BYTE"
            }

            Size::Word=>{
                "WORD"
            }

            Size::Dword=>{
                "DWORD"
            }

            Size::Qword=>{
                "QWORD"
            }
        };

        let mem_size = tape.cell_count * cell_size;

        if operation.token_type != TokenType::NewLine{

            file_content.push_str(format!("    ; ---------   {:?} x {} ( {} )  --------- ;\n",operation.token_type,operation.count, tape.name).as_str());

        }

        match operation.token_type {
            // '>' => {
            TokenType::PointerRight => {
                // file_content.push_str("    add QWORD[pointer], 1\n");

                //check if pointer is at the end

                file_content
                    .push_str(format!("    cmp QWORD[{}_pointer], {}\n",tape.name, mem_size - cell_size).as_str());
                file_content.push_str(format!("    je .bound_{}\n", i).as_str());
                file_content.push_str(format!("    add QWORD[{}_pointer], {}\n",tape.name, operation.count*cell_size).as_str());
                file_content.push_str(format!("    jmp .skip_{}\n", i).as_str());
                file_content.push_str(format!("    .bound_{}:\n", i).as_str());
                file_content.push_str(format!("        mov QWORD[{}_pointer], 0\n", tape.name).as_str());
                file_content.push_str(format!("    .skip_{}:\n", i).as_str());
            }
            // '<' => {
            TokenType::PointerLeft => {
                //check if pointer is zero
                file_content.push_str(format!("    cmp QWORD[{}_pointer], 0\n", tape.name).as_str());
                file_content.push_str(format!("    je .bound_{}\n", i).as_str());
                file_content.push_str(format!("    sub QWORD[{}_pointer], {}\n",tape.name, operation.count*cell_size).as_str());
                file_content.push_str(format!("    jmp .skip_{}\n", i).as_str());
                file_content.push_str(format!("    .bound_{}:\n", i).as_str());
                file_content
                    .push_str(format!("        mov QWORD[{}_pointer], {}\n", tape.name , mem_size - cell_size).as_str());
                file_content.push_str(format!("    .skip_{}:\n", i).as_str());

                // file_content.push_str("    sub QWORD[pointer], 1\n");
            }
            // '$' => {
            TokenType::MemAddr => {
                // put current mem addr into cell
                // file_content.push_str(format!("debug_mem_{}:\n", mem_dbg_ln).as_str());
                file_content.push_str(format!("    mov rax, {}\n", tape.name).as_str());
                file_content.push_str(format!("    add rax, QWORD[{}_pointer]\n",tape.name).as_str());
                file_content.push_str("    mov rbx, rax\n");
                file_content.push_str("    mov QWORD[rax], rbx\n");
                // mem_dbg_ln += 1;
            }

            // '%' => {
            TokenType::BaseMemAddr => {
                // put base mem addr into cell
                file_content.push_str(format!("    mov rax, {}\n", tape.name).as_str());
                file_content.push_str(format!("    add rax, QWORD[{}_pointer]\n",tape.name).as_str());
                file_content.push_str(format!("    mov rbx, {}\n", tape.name).as_str());
                file_content.push_str("    mov QWORD[rax], rbx\n");
            }

            // '&' => {
            TokenType::PointerReset => {
                // set pointer to 0
                file_content.push_str(format!("    mov QWORD[{}_pointer], 0\n", tape.name).as_str());
            }

            // '?' => {
            TokenType::Syscall => {
                // perform syscall
                file_content.push_str(format!("    mov rbp, {}\n", tape.name).as_str());
                file_content.push_str(format!("    add rbp, QWORD[{}_pointer]\n", tape.name).as_str());
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
                file_content.push_str(format!("   mov rax, {}\n",tape.name).as_str());
                file_content.push_str(format!("   add rax, QWORD[{}_pointer]\n",tape.name).as_str());
                file_content.push_str(format!("   mov {}[rax], 0\n",cell_size_str).as_str());
            }
            
            // '+' => {
                TokenType::Add => {
                    file_content.push_str(format!("    mov rax, {}\n",tape.name).as_str());
                    file_content.push_str(format!("    add rax, QWORD[{}_pointer]\n",tape.name).as_str());
                    file_content.push_str(format!("    add {} [rax], {}\n",cell_size_str,operation.count).as_str());
                }
                
                TokenType::IntLit => {
                    // println!("Not implemented yet");
                    // exit(1);
                    file_content.push_str(format!("    mov rax, {}\n",tape.name).as_str());
                    file_content.push_str(format!("    add rax, QWORD[{}_pointer]\n",tape.name).as_str());
                    file_content.push_str(format!("    mov {}[rax], {}\n",cell_size_str,operation.values[0]).as_str());
                }
                
                TokenType::StringLit => {
                    file_content.push_str(format!("    mov rax, {}\n",tape.name).as_str());
                    file_content.push_str(format!("    add rax, QWORD[{}_pointer]\n",tape.name).as_str());
                    
                    let str_val = operation.values[0].clone();
                    let len = str_val.len();
                    for n in 0..len{
                        let val = str_val.chars().nth(n).unwrap();
                        file_content.push_str(format!("    mov BYTE [rax], {}\n",val as u8).as_str());
                        file_content.push_str("    add rax, 1\n");
                    }


                }

                TokenType::Push => {
                file_content.push_str(format!("    mov rax, {}\n",tape.name).as_str());
                file_content.push_str(format!("    add rax, QWORD[{}_pointer]\n",tape.name).as_str());
                file_content.push_str("    mov rbx, 0\n");

                let rbx = match tape.size{
                    Size::Byte => {
                        "bl"
                    }

                    Size::Word => {
                        "bx"
                    }

                    Size::Dword => {
                        "ebx"
                    }

                    Size::Qword => {
                        "rbx"
                    }
                };

                file_content.push_str(format!("    mov {rbx}, {}[rax]\n",cell_size_str).as_str());
                for _ in 0..operation.count{
                    file_content.push_str("    push rbx\n");
                }
            }

            TokenType::Pop => {
                file_content.push_str(format!("    mov rax, {}\n",tape.name).as_str());
                file_content.push_str(format!("    add rax, QWORD[{}_pointer]\n",tape.name).as_str());
                file_content.push_str("    mov rbx, 0\n");
                for _ in 0..operation.count{
                    file_content.push_str("    pop rbx\n");
                }

                let rbx = match tape.size{
                    Size::Byte => {
                        "bl"
                    }

                    Size::Word => {
                        "bx"
                    }

                    Size::Dword => {
                        "ebx"
                    }

                    Size::Qword => {
                        "rbx"
                    }
                };

                file_content.push_str(format!("    mov {}[rax], {rbx}\n",cell_size_str).as_str());
            }

            // '-' => {
            TokenType::Sub => {
                file_content.push_str(format!("    mov rax, {}\n",tape.name).as_str());
                file_content.push_str(format!("    add rax, QWORD[{}_pointer]\n",tape.name).as_str());
                file_content.push_str(format!("    sub {}[rax], {}\n",cell_size_str,operation.count).as_str());
            }
            // '.' => {
            TokenType::WriteByte => {
                file_content.push_str("    mov rax, 1\n");
                file_content.push_str("    mov rdi, 1\n");
                file_content.push_str(format!("    mov rsi, {}\n",tape.name).as_str());
                file_content.push_str(format!("    add rsi, QWORD[{}_pointer]\n",tape.name).as_str());
                
                //TODO: handle writing bigger tapes properly
                
                file_content.push_str("    mov rdx, 1\n");
                file_content.push_str("    syscall\n");
            }
            // ',' => {
            TokenType::ReadByte => {
                file_content.push_str("    mov rax, 0\n");
                file_content.push_str("    mov rdi, 0\n");
                file_content.push_str(format!("    mov rsi, {}\n",tape.name).as_str());
                file_content.push_str(format!("    add rsi, QWORD[{}_pointer]\n",tape.name).as_str());
                
                //TODO: handle writing bigger tapes properly

                file_content.push_str("    mov rdx, 1\n");
                file_content.push_str("    syscall\n");
            }
            // '[' => {
            TokenType::ZeroJump => {
                file_content.push_str(format!("    mov rax, {}\n",tape.name).as_str());
                file_content.push_str(format!("    add rax, QWORD[{}_pointer]\n",tape.name).as_str());
                file_content.push_str("    mov rdx, 0\n");

                let rdx = match tape.size{
                    Size::Byte => {
                        "dl"
                    }

                    Size::Word => {
                        "dx"
                    }

                    Size::Dword => {
                        "edx"
                    }

                    Size::Qword => {
                        "rdx"
                    }
                };

                file_content.push_str(format!("    mov {rdx}, {}[rax]\n",cell_size_str).as_str());
                file_content.push_str("    cmp rdx, 0\n");

                let mut condition_id = 0;
                for (j, jump) in jumps.iter().enumerate().take(len).skip(last_condition) {
                    if matches!(jump, Jumps::Condition(_)) {
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

                file_content.push_str(format!("    je .forward_{}\n", forward_id).as_str());
                file_content.push_str(format!("    .condition_{}:\n", condition_id).as_str());
            }
            // ']' => {
            TokenType::NonZeroJump => {
                file_content.push_str(format!("    mov rax, {}\n",tape.name).as_str());
                file_content.push_str(format!("    add rax, QWORD[{}_pointer]\n",tape.name).as_str());
                file_content.push_str("    mov rdx, 0\n");

                let rdx = match tape.size{
                    Size::Byte => {
                        "dl"
                    }

                    Size::Word => {
                        "dx"
                    }

                    Size::Dword => {
                        "edx"
                    }

                    Size::Qword => {
                        "rdx"
                    }
                };

                file_content.push_str(format!("    mov {rdx}, {}[rax]\n",cell_size_str).as_str());
                file_content.push_str("    cmp rdx, 0\n");

                let mut forward_id = 0;
                for (j, jump) in jumps.iter().enumerate().take(len).skip(last_condition) {
                    if matches!(jump, Jumps::Forward(_)) {
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

                file_content.push_str(format!("    jne .condition_{}\n", condition_id).as_str());
                file_content.push_str(format!("    .forward_{}:\n", forward_id).as_str());
            }
            TokenType::CurrentTape =>{

                
                if let Some(tape) = operation.tape.clone() {

                    for (n,tapem) in tapes.iter().enumerate(){
                        if tapem.name == tape.name{
                            current_tape = n;
                            break;
                        }
                    }

                }else{
                    println!("Unreachable something is wrong with pareser");
                    exit(1);
                }

                // println!("CurrentTape: Not implemented yet");
                // exit(1);
            }
            TokenType::NewLine => {}
            TokenType::MacroDecl => {
                println!("MacroDecl: Unreachable Something with preprocessing is wrong");
                exit(1);
            }
            TokenType::TapeDecl => {
                println!("TapeDecl: Unreachable Something with preprocessing is wrong");
                exit(1);
            }

            TokenType::TapeName => {
                // println!("TapeName: Unreachable Something with preprocessing is wrong");
                // exit(1);

                let mut exist = false;
                for tape in tapes{
                    if tape.name == operation.values[0]{
                        exist = true;
                    }
                }

                if !exist{
                    println!("Tape {} isnt defined", operation.values[0]);
                    exit(1);
                }

                file_content.push_str(format!("    mov rax, {}\n", tape.name).as_str());
                file_content.push_str(format!("    add rax, QWORD[{}_pointer]\n",tape.name).as_str());
                file_content.push_str(format!("    mov rbx, {}\n", operation.values[0]).as_str());
                file_content.push_str("    mov QWORD[rax], rbx\n");
            }

            TokenType::CellSize => {
                println!("CellSize: Unreachable Something with preprocessing is wrong");
                exit(1);
            }
            TokenType::IfdefMacro => {
                println!("IfdefMacro: Unreachable Something with preprocessing is wrong");
                exit(1);
            }
            TokenType::IfNdefMacro => {
                println!("IfNdefMacro: Unreachable Something with preprocessing is wrong");
                exit(1);
            }
            TokenType::ElseMacro => {
                println!("ElseMacro: Unreachable Something with preprocessing is wrong");
                exit(1);
            }
            TokenType::EndifMacro => {
                println!("EndifMacro: Unreachable Something with preprocessing is wrong");
                exit(1);
            }
            TokenType::IncludeMacro => {
                println!("IncludeMacro: Unreachable Something with preprocessing is wrong");
                exit(1);
            }
            TokenType::IncludePath => {
                println!("IncludePath: Unreachable Something with preprocessing is wrong");
                exit(1);
            }
            TokenType::Ident => {
                println!("Idents are not implemented yet");
                exit(1);
            }

            // _ => {
            //     println!("Unreachable: Token {}", token.value);
            // }
        }
    }
}
