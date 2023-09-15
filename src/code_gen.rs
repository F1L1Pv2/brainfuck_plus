use std::process::exit;

use crate::common::MEM_SIZE;



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

fn cross_reference(contents: &String) -> Vec<Jumps> {
    let len = contents.len();

    let mut jumps = Vec::new();

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
                // match jumps[last_condition] {
                //     Jumps::Condition(ref mut condition) => {
                //         condition.addr = len;
                //     }
                //     _ => {}
                // }

                if let Jumps::Condition(ref mut condition) = jumps[last_condition]{
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

pub fn generate_code(contents: String, file_content: &mut String) {
    let mut last_condition = 0;
    let mut mem_dbg_ln = 0;

    let len = contents.len();

    let jumps = cross_reference(&contents);

    for i in 0..len {
        let ch = contents.chars().nth(i).unwrap();

        match ch {
            '>' => {
                // file_content.push_str("    add QWORD[pointer], 1\n");

                //check if pointer is at the end
                file_content
                    .push_str(format!("    cmp QWORD[pointer], {}\n", MEM_SIZE - 1).as_str());
                file_content.push_str(format!("    je bound_{}\n", i).as_str());
                file_content.push_str("    add QWORD[pointer], 1\n");
                file_content.push_str(format!("    jmp skip_{}\n", i).as_str());
                file_content.push_str(format!("bound_{}:\n", i).as_str());
                file_content.push_str("    mov QWORD[pointer], 0\n");
                file_content.push_str(format!("skip_{}:\n", i).as_str());
            }
            '<' => {
                //check if pointer is zero
                file_content.push_str("    cmp QWORD[pointer], 0\n");
                file_content.push_str(format!("    je bound_{}\n", i).as_str());
                file_content.push_str("    sub QWORD[pointer], 1\n");
                file_content.push_str(format!("    jmp skip_{}\n", i).as_str());
                file_content.push_str(format!("bound_{}:\n", i).as_str());
                file_content
                    .push_str(format!("    mov QWORD[pointer], {}\n", MEM_SIZE - 1).as_str());
                file_content.push_str(format!("skip_{}:\n", i).as_str());

                // file_content.push_str("    sub QWORD[pointer], 1\n");
            }
            '$' => {
                // put current mem addr into cell
                file_content.push_str(format!("debug_mem_{}:\n", mem_dbg_ln).as_str());
                file_content.push_str("    mov rax, mem\n");
                file_content.push_str("    add rax, QWORD[pointer]\n");
                file_content.push_str("    mov rbx, rax\n");
                file_content.push_str("    mov QWORD[rax], rbx\n");
                mem_dbg_ln += 1;
            }

            '%' => {
                // put base mem addr into cell
                file_content.push_str("    mov rax, mem\n");
                file_content.push_str("    add rax, QWORD[pointer]\n");
                file_content.push_str("    mov rbx, mem\n");
                file_content.push_str("    mov QWORD[rax], rbx\n");
            }

            '&' => {
                // set pointer to 0
                file_content.push_str("    mov QWORD[pointer], 0\n");
            }

            '?' => {
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

            '\'' => {
                // clear current cell
                file_content.push_str("   mov rax, mem\n");
                file_content.push_str("   add rax, QWORD[pointer]\n");
                file_content.push_str("   mov BYTE[rax], 0\n");
            }

            '+' => {
                file_content.push_str("    mov rax, mem\n");
                file_content.push_str("    add rax, QWORD[pointer]\n");
                file_content.push_str("    add BYTE [rax], 1\n");
            }
            '-' => {
                file_content.push_str("    mov rax, mem\n");
                file_content.push_str("    add rax, QWORD[pointer]\n");
                file_content.push_str("    sub BYTE [rax], 1\n");
            }
            '.' => {
                file_content.push_str("    mov rax, 1\n");
                file_content.push_str("    mov rdi, 1\n");
                file_content.push_str("    mov rsi, mem\n");
                file_content.push_str("    add rsi, QWORD[pointer]\n");
                file_content.push_str("    mov rdx, 1\n");
                file_content.push_str("    syscall\n");
            }
            ',' => {
                file_content.push_str("    mov rax, 0\n");
                file_content.push_str("    mov rdi, 0\n");
                file_content.push_str("    mov rsi, mem\n");
                file_content.push_str("    add rsi, QWORD[pointer]\n");
                file_content.push_str("    mov rdx, 1\n");
                file_content.push_str("    syscall\n");
            }
            '[' => {
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
            ']' => {
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
            _ => {
                if !ch.is_whitespace() {
                    println!("Unsupported command: {}", ch);
                    exit(1);
                }
            }
        }
    }
    
}
