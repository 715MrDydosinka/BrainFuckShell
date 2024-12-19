use crate::Executable;

use std::collections::HashMap;

pub struct Lvars {
    pub local_vars:HashMap<String,String>
}

impl Lvars {

    pub fn new() -> Self{
        Lvars { local_vars: HashMap::new() }
    }

    fn lvar_print_all(&mut self) {
        if self.local_vars.is_empty() {
            println!("[Empty]");
            return
        }
        for (key, value) in &self.local_vars{
            println!("'{}' : '{}'", key, value)
        }
    }

    fn lvar_print_one(&mut self, key:&str) {
        if let Some(value) = self.local_vars.get(key) {
            println!("'{}' : '{}'", key, value);
        } else {
            println!("Variable '{}' is missing.", key);
        }
    }

    fn lvar_delete(&mut self) {
    }

    fn lvar_add(&mut self, key:&str, value:&str) {
        self.local_vars.insert(key.to_string(), value.to_string());
    }

    fn run(&mut self, args:Vec<&str>) -> Option<String> {
        match args.len() {
            0 => self.lvar_print_all(),
            1 => self.lvar_print_one(args[0]),
            2 => self.lvar_add(args[0], args[1]),
            _ => return Some("Too many args for command 'lvar'".to_owned())

        };

        None
    }


}

impl Executable for Lvars {

    fn exec(&mut self, args:Vec<&str>) -> u8 {
        

        if let Some(err) = self.run(args){
            eprintln!("{}",err);
            return 1
        };

        0
    }


}