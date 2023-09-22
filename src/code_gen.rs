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

pub fn generate_code(operations: Vec<Operation>, file_content: &mut String) {
    let mut last_condition = 0;
    let mut mem_dbg_ln = 0;

    let len = operations.len();
    
    let jumps = cross_reference(&operations);
    // let jumps = cross_reference(&operations);
    
    for (i,operation) in operations.iter().enumerate() {
        // for i in 0..len {
        //     let ch = contents.chars().nth(i).unwrap();

        match operation.token_type {
            // '>' => {
            TokenType::PointerRight => {
                // file_content.push_str("    add QWORD[pointer], 1\n");

                //check if pointer is at the end
                file_content
                    .push_str(format!("    cmp QWORD[pointer], {}\n", MEM_SIZE - 1).as_str());
                file_content.push_str(format!("    je bound_{}\n", i).as_str());
                file_content.push_str(format!("    add QWORD[pointer], {}\n", operation.count).as_str());
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
                file_content.push_str(format!("    sub QWORD[pointer], {}\n", operation.count).as_str());
                file_content.push_str(format!("    jmp skip_{}\n", i).as_str());
                file_content.push_str(format!("bound_{}:\n", i).as_str());
                file_content
                    .push_str(format!("    mov QWORD[pointer], {}\n", MEM_SIZE - 1).as_str());
                file_content.push_str(format!("skip_{}:\n", i).as_str());

                // file_content.push_str("    sub QWORD[pointer], 1\n");
            }
            // '$' => {
            TokenType::MemAddr => {
                // put current mem addr into cell
                file_content.push_str(format!("debug_mem_{}:\n", mem_dbg_ln).as_str());
                file_content.push_str("    mov rax, mem\n");
                file_content.push_str("    add rax, QWORD[pointer]\n");
                file_content.push_str("    mov rbx, rax\n");
                file_content.push_str("    mov QWORD[rax], rbx\n");
                mem_dbg_ln += 1;
            }

            // '%' => {
            TokenType::BaseMemAddr => {
                // put base mem addr into cell
                file_content.push_str("    mov rax, mem\n");
                file_content.push_str("    add rax, QWORD[pointer]\n");
                file_content.push_str("    mov rbx, mem\n");
                file_content.push_str("    mov QWORD[rax], rbx\n");
            }

            // '&' => {
            TokenType::PointerReset => {
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
                    file_content.push_str(format!("    add BYTE [rax], {}\n",operation.count).as_str());
                }
                
                TokenType::IntLit => {
                    // println!("Not implemented yet");
                    // exit(1);
                    file_content.push_str("    mov rax, mem\n");
                    file_content.push_str("    add rax, QWORD[pointer]\n");
                    file_content.push_str(format!("    mov BYTE [rax], {}\n",operation.values[0]).as_str());
                }
                
                TokenType::StringLit => {
                    file_content.push_str("    mov rax, mem\n");
                    file_content.push_str("    add rax, QWORD[pointer]\n");
                    
                    let str_val = operation.values[0].clone();
                    let len = str_val.len();
                    for n in 0..len{
                        let val = str_val.chars().nth(n).unwrap();
                        file_content.push_str(format!("    mov BYTE [rax], {}\n",val as u8).as_str());
                        file_content.push_str("    add rax, 1\n");
                    }


                }

                TokenType::Push => {
                file_content.push_str("    mov rax, mem\n");
                file_content.push_str("    add rax, QWORD[pointer]\n");
                file_content.push_str("    mov rbx, 0\n");
                file_content.push_str("    mov bl, byte[rax]\n");
                for _ in 0..operation.count{
                    file_content.push_str("    push rbx\n");
                }
            }

            TokenType::Pop => {
                file_content.push_str("    mov rax, mem\n");
                file_content.push_str("    add rax, QWORD[pointer]\n");
                file_content.push_str("    mov rbx, 0\n");
                for _ in 0..operation.count{
                    file_content.push_str("    pop rbx\n");
                }
                file_content.push_str("    mov byte[rax], bl\n");
            }

            // '-' => {
            TokenType::Sub => {
                file_content.push_str("    mov rax, mem\n");
                file_content.push_str("    add rax, QWORD[pointer]\n");
                file_content.push_str(format!("    sub BYTE [rax], {}\n",operation.count).as_str());
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

                file_content.push_str(format!("    jne condition_{}\n", condition_id).as_str());
                file_content.push_str(format!("forward_{}:\n", forward_id).as_str());
            }
            TokenType::NewLine => {}
            TokenType::MacroDecl => {
                println!("MacroDecl: Unreachable Something with preprocessing is wrong");
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
            
            TokenType::OpenParen => {
                println!("OpenParen: Not implemented yet!");
                exit(1);
            }
            TokenType::CloseParen => {
                println!("CloseParen: Not implemented yet!");
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
