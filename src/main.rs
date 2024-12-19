use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::collections::{HashMap, VecDeque};
use std::env;
use regex::Regex;

mod suicide;
use suicide::roulete;

struct Shell{
    //Just a placeholder for this moment
    args: Vec<String>,

    current_dir: PathBuf,

    local_vars:HashMap<String,String>,

    suicide_mode: bool,
    dummy_mode: bool
}

impl Shell {

    pub fn parse_args(args: Vec<String>) -> Self{
        Shell{
            args:args,
            current_dir: env::current_dir().unwrap(),
            local_vars:HashMap::new(),
            suicide_mode: false,
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
            return Some("Too many args for command 'cd'".to_owned())
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

    fn evar(args: Vec<&str>) -> Option<String> {

        match args.len() {
            0 => {
                for (key, value) in env::vars() {
                    println!("'{}' : '{}'", key, value);
                }
            }
            1 => { 
                match env::var(args[0]) {
                    Ok(value) => println!("'{}' : '{}'", args[0], value),
                    Err(e) => println!("Couldn't read {}: {}", args[0], e),
                }
            }
            2 => {
                unsafe { env::set_var(args[0], args[1]); }
            }
            _ => return Some("Too many args for command 'set'".to_owned())

        }

        None

    }

    fn lvar(&mut self, args: Vec<&str>) -> Option<String> {
        match args.len() {
            0 => {
                println!("Local variables:");
                if self.local_vars.is_empty() {
                    println!("[Empty]");
                    return None
                }
                for (key, value) in &self.local_vars{
                    println!("'{}' : '{}'", key, value)
                }
            }
            1 => { 
                if let Some(value) = self.local_vars.get(args[0]) {
                    println!("'{}' : '{}'", args[0], value);
                } else {
                    println!("Variable '{}' is missing.", args[0]);
                }
            }
            2 => {
                self.local_vars.insert(args[0].to_owned(), args[1].to_owned());
            }
            _ => return Some("Too many args for command 'set'".to_owned())

        }

        None
    }

    fn cm(&mut self) -> Option<String> {
        self.dummy_mode = !self.dummy_mode;
        None
    }

    fn execute_extenal(&mut self, command: &str, args: Vec<&str>) -> Option<String> {
        //println!("{}, {:?}", command, args);
        match Command::new(command).args(&args).spawn() {
            Ok(mut child) => {
                if let Err(e) = child.wait() {
                    return Some(format!("Error while waiting for command to finish: {}", e).to_owned());
                }
            }
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    if self.suicide_mode { roulete(); }
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

            let (command, args) = Shell::split_prompt(&prompt);
            
            let err: Option<String> = match command{
                "exit"   => break,
                "help"   => Shell::help(),
                "cd"     => self.cd(args),
                "evar"    => Shell::evar(args),
                "lvar"    => self.lvar(args),
                "export" => Shell::evar(args),
                "cm"     => self.cm(),
                _ => Shell::execute_extenal(self,command, args)       
            };

            if let Some(err) = err {
                eprintln!("{}", err);
            }
        }

        Shell::motn();
    }

}



fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut shell = Shell::parse_args(args);
    shell.start();
}
