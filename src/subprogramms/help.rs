use crate::Executable;
use crate::LocalVars;
pub struct Help { }

impl Help {

    fn main_help() {
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

    fn run(args:Vec<&str>) -> Option<String> {
        
        if args.is_empty() {
            Help::main_help();
        }

        if args.len() > 1 {
            return Some("Too many args for command 'help'".to_owned())
        }

        None
    }

}

impl Executable for Help {
    
    fn exec(args:Vec<&str>, localvars: &mut LocalVars) -> u8 {
        let _ = localvars;

        if let Some(err) = Help::run(args){
            eprintln!("{}",err);
            return 1
        };

        0
    }

}