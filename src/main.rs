use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::collections::VecDeque;
use std::env;
use regex::Regex;

struct Shell{
    args: Vec<String>,
    //Placeholder
    current_dir: PathBuf,

    dummy_mode: bool
}

impl Shell {

    pub fn parse_args(args: Vec<String>) -> Self{
        Shell{
            args:args,
            current_dir: env::current_dir().unwrap(),
            dummy_mode: true
        }
    }
    
    fn get_prompt(&self) -> String{
        let username = env::var_os("USER").map(|os_str| os_str.to_string_lossy().into_owned())
        .unwrap_or_else(|| "@NULL".to_string());

        let start: String;
        if self.dummy_mode{
            start = "Dummy ".to_owned();
        }
        else {
            start = "BF'ed ".to_owned()
        }
        return format!("{}{} > ", start, username);
    }

    fn motd() {
        println!("Glad to see that you're using BrainFuckShell by Hlupa \nRemember, you're not welcome here.\nIf you need some help, type 'help' :)")
    }

    fn motn() {
        println!("Bye")
    }

    fn interpret(code: &str) -> Result<String, String> {
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
                                return Err("Unmatched '[' in the command.".to_owned());
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
                        return Err("Unmatched ']' in the command.".to_owned());
                    }
                }
                _ => {}
            }fn split_by_spaces_with_quotes(input: &str) -> Vec<String> {
                let re = Regex::new(r#""([^"]*)"|'([^']*)'|(\S+)"#).unwrap();
                re.captures_iter(input)
                    .map(|cap| {
                        cap.iter()
                            .filter_map(|m| m.map(|m| m.as_str().to_string()))
                            .next()
                            .unwrap_or_default()
                    })
                    .collect()
            }
            
            instruction_ptr += 1;
        }
        //println!("{:?}", memory);
        let _result:String = result.into_iter().collect(); 
        if _result.is_empty() {
            return Err("".to_owned())
        }
        Ok(_result)
    }

    fn help() -> Option<String> {

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
        None
    }

    fn cd(&mut self, args: Vec<&str>) -> Option<String> {
        if args.is_empty() {
            return Some("No directory specified".to_owned())
        }
        if args.len() > 1 {
            return Some("Too many args for cd command".to_owned())
        }


        let new_dir = if args[0] == ".." {
            self.current_dir.parent().unwrap_or_else(|| &self.current_dir).to_path_buf()
        } else {
            let new_dir = self.current_dir.join(args[0]);
            if new_dir.exists() && new_dir.is_dir() {
                new_dir
            } else {
                return Some("Directory not found".to_owned());
            }
        };

        self.current_dir = new_dir;
        if env::set_current_dir(&self.current_dir).is_err() {
            return Some("Failed to change directory".to_owned())
        };

        return None
    }

    fn export(args: Vec<&str>) -> Option<String> {
        todo!("TODO export")
    }

    fn cm(&mut self) -> Option<String> {
        self.dummy_mode = !self.dummy_mode;
        None
    }

    fn execute_extenal(command: &str, args: Vec<&str>) -> Option<String> {
        println!("{}, {:?}", command, args);
        match Command::new(command).args(&args).spawn() {
            Ok(mut child) => {
                if let Err(e) = child.wait() {
                    return Some(format!("Error while waiting for command to finish: {}", e).to_owned());
                }
            }
            Err(e) => {
                return Some(format!("Error executing command '{}': {}", command, e).to_owned());
            }
        }

        None
    }

    fn split_prompt(input: &str) -> (&str, Vec<&str>) {
        let re = Regex::new(r#""([^"]*)"|'([^']*)'|(\S+)"#).unwrap();
        let captured: Vec<&str> = re.captures_iter(input)
            .map(|cap| {
                cap.iter()
                    .filter_map(|m| m.map(|m| m.as_str()))
                    .next()
                    .unwrap_or_default()
            })
            .collect();
        
        if captured.is_empty() {
            ("", Vec::new())
        } else {
            let command = captured[0];
            let args = captured[1..].to_vec();
            (command, args)
        }
    }
     
    fn start(&mut self){
        Shell::motd();
 
        loop {
            print!("{}", self.get_prompt());
            io::stdout().flush().unwrap();

            // Read input from the user
            let mut raw_input = String::new();
            if io::stdin().read_line(&mut raw_input).is_err() {
                eprintln!("Failed to read input.");
                continue;
            }

            let prompt: String;
            
            if !self.dummy_mode{
                prompt = match Shell::interpret(&raw_input) {
                    Ok(value) => value,
                    Err(err) => { 
                        if !err.trim().is_empty() { 
                            eprintln!("{}",err); 
                        }
                        continue;
                    }
                };
                println!("Entered command: {}", prompt);
            }
            else {
                prompt = raw_input;
                if prompt.trim().is_empty() {
                    continue;
                }
            }

            let (command, args) = Shell::split_prompt(prompt.trim());
            

            /*let mut parts = prompt.trim().split_whitespace();
            let command = match parts.next() {
                Some(cmd) => cmd,
                None => continue,
            };
            let args: Vec<&str> = parts.collect();
            */
            let err: Option<String> = match command{
                "exit"   => break,
                "help"   => Shell::help(),
                "cd"     => self.cd(args),
                "export" => Shell::export(args),
                "cm"     => self.cm(),
                _ => Shell::execute_extenal(command, args)       
            };

            if let Some(err) = err {
                eprintln!("{}", err);
            }
        }

        Shell::motn();
    }

}



fn main() {
    let args: Vec<String> = env::args().collect();

    let mut shell = Shell::parse_args(args);
    shell.start();
}
