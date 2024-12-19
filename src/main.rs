


use std::io::{self, Read, Write};
use std::process::Command;
use std::collections::VecDeque;
use std::env;

fn shell_prompt() -> String{
    let username = env::var_os("USER")
        .map(|os_str| os_str.to_string_lossy().into_owned())
        .unwrap_or_else(|| "@NULL".to_string());

    return format!("BF'ed {} > ", username);
}

fn motd() {
    println!("Glad to see that you're using BrainFuckShell by Hlupa \nRemember, you're not welcome here.\n")
}

fn motn() {
    println!("Bye")
}

fn help() {

    let help = 
    "BrainFuck Shell - Is a command-line interactive shell where all commands must be written in the Brainfuck language.
    
    Supported symbols:
       > - Increment the data pointer by one (to point to the next cell to the right).
       < - Decrement the data pointer by one (to point to the next cell to the left).
       + - Increment the byte at the data pointer by one.
       - - Decrement the byte at the data pointer by one.
       . - Output the byte at the data pointer.
       [ - If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command. 
       ] - If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.
    
    Example:
    'fastfetch' - ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>
     ";

    println!("{}", help);
}

fn interpret(code: &str) -> String {
    let mut memory = vec![0u8; 1024];
    let mut ptr = 0;
    let mut instruction_ptr = 0;
    let code: Vec<char> = code.chars().collect();
    let mut loop_stack = VecDeque::new();

    let mut result: Vec<char> = Vec::new();

    while instruction_ptr < code.len() {
        match code[instruction_ptr] {
            '>' => {
                ptr = (ptr + 1) % memory.len();
            }
            '<' => {
                ptr = (ptr + memory.len() - 1) % memory.len();
            }
            '+' => {
                memory[ptr] = memory[ptr].wrapping_add(1);
            }
            '-' => {
                memory[ptr] = memory[ptr].wrapping_sub(1);
            }
            '.' => {
                result.push(memory[ptr] as char);
            }        
            '[' => {
                if memory[ptr] == 0 {
                    let mut open_loops = 1;
                    while open_loops > 0 {
                        instruction_ptr += 1;
                        if instruction_ptr >= code.len() {
                            eprintln!("Unmatched '[' in the code.");
                            return "".to_owned();
                        }
                        if code[instruction_ptr] == '[' {
                            open_loops += 1;
                        } else if code[instruction_ptr] == ']' {
                            open_loops -= 1;
                        }
                    }
                } else {
                    loop_stack.push_back(instruction_ptr);
                }
            }
            ']' => {
                if let Some(start_loop) = loop_stack.back() {
                    if memory[ptr] != 0 {
                        instruction_ptr = *start_loop;
                    } else {
                        loop_stack.pop_back();
                    }
                } else {
                    eprintln!("Unmatched ']' in the code.");
                    return "".to_owned();
                }
            }
            _ => {}
        }
        instruction_ptr += 1;
    }
    //println!("{:?}", memory);
    return result.into_iter().collect();
}

fn main() {

    motd();
 
    loop {
        print!("{}", shell_prompt());
        io::stdout().flush().unwrap();

        // Read input from the user
        let mut raw_input = String::new();
        if io::stdin().read_line(&mut raw_input).is_err() {
            eprintln!("Failed to read input.");
            continue;
        }

        let result = interpret(&raw_input);
        
        if result.is_empty() {
            println!("");
            continue;
        }
        print!("Entered command: <{}>\n", result);

        let input = result.trim();
       
        match input{
            "exit" => { break; },
            "help" => { help(); continue; }
            _ => {}
        }

        let mut parts = input.split_whitespace();
        let command = match parts.next() {
            Some(cmd) => cmd,
            None => continue,
        };
        let args: Vec<&str> = parts.collect();

        match Command::new(command).args(&args).spawn() {
            Ok(mut child) => {
                if let Err(e) = child.wait() {
                    eprintln!("Error while waiting for command to finish: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error executing command '{}': {}", command, e);
            }
        }
    }

    motn();
}
