use std::io::{self, Stdout, Write};
use std::process::{Command, Stdio};
use std::collections::VecDeque;
use std::env;
use cd::CD;
use evar::Evars;
use help::Help;
use local_vars::{GetValue, LocalVars};
//use local_vars;
//use lvar::Lvars;
use regex::Regex;


mod suicide;
use subprogramms::{*};
use suicide::roulete;

mod subprogramms {
    pub mod evar;
    pub mod help;
    pub mod cd;
}

mod local_vars;

pub trait Executable {
    fn exec(args:Vec<&str>, localvars: &mut LocalVars) -> u8;
}

struct Shell{
    //Just a placeholder for this moment
    #[allow(dead_code)] 
    args: Vec<String>,
 
    local_vars: LocalVars,
}

impl Shell {

    pub fn parse_args(args: Vec<String>) -> Self{
        Shell{
            args:args,
            local_vars  : LocalVars::new()
        }
    }
    
    fn get_prompt(&self) -> String{
        let username = env::var_os("USER").map(|os_str| os_str.to_string_lossy().into_owned())
        .unwrap_or_else(|| "@NULL".to_string());

        let start = if self.local_vars.get_bool("dummy_mode").unwrap_or(false) {
            "Dummy ".to_owned()
        } else {
            "BF'ed ".to_owned()
        };

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

    fn execute_extenal(&mut self, command: &str, args: Vec<&str>) -> Option<String> {
        //println!("{}, {:?}", command, args);
        match Command::new(command).args(&args).stdout(Stdio::inherit()).stderr(Stdio::inherit()).spawn() {
            Ok(mut child) => {
                match child.wait() {
                    Ok(status) => {
                        if let Some(code) = status.code() {
                            println!("Process exited with code: {}", code);
                        } else {
                            println!("Process terminated by signal");
                        }
                    }
                    Err(e) => {
                        return Some(format!("Error while waiting for command to finish: {}", e).to_owned());
                    }
                }
            }
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {

                    if self.local_vars.get_bool("suicide_mode") == Some(true) {
                        roulete();
                    }

                    return Some(format!("Command not found: '{}'", command).to_owned());
                } else {
                    return Some(format!("Error executing command '{}': {}", command, e).to_owned());
                }
            }
        } 

        None
    }

    fn split_prompt(input: &str) -> (&str, Vec<&str>) {
        let re = Regex::new(r#""([^"]*)"|'([^']*)'|(\S+)"#).unwrap(); 
        let mut iter = re.captures_iter(input);
        let first = iter.next().map(|cap| cap.get(0).unwrap().as_str()).unwrap_or("");
        let rest = iter
            .map(|cap| cap.get(0).unwrap().as_str().trim_matches(&['\'', '"'][..]))
            .collect::<Vec<&str>>();

        (first, rest)
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


            if self.local_vars.get_bool("dummy_mode") == Some(true) {
                prompt = raw_input;
                if prompt.trim().is_empty() {
                    continue;
                }
            }
            else {
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

            let (command, args) = Shell::split_prompt(&prompt);
            
            let _:u8 = match command{
                "exit"   => break,
                "help"   => Help::exec (args, &mut self.local_vars),
                "cd"     => CD::exec   (args, &mut self.local_vars),
                "evar"   => Evars::exec(args, &mut self.local_vars),
                _ => { Shell::execute_extenal(self,command, args); 0}
            };
        }

        Shell::motn();
    }

}



fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut shell = Shell::parse_args(args);
    shell.start();
}

