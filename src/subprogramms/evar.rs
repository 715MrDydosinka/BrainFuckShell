use std::env;
use std::env::VarError;

use crate::Executable;
use crate::LocalVars;

pub struct Evars { }

impl Evars {

    pub fn evar_get_one(key:&str) -> Result<String, VarError>{
        match env::var(key) {
            Ok(value) => Ok(value),
            Err(e) => Err(e)
        }
    }

    fn evar_print_all() {
        for (key, value) in env::vars() {
            println!("'{}' : '{}'", key, value);
        }
    }

    fn evar_print_one(key:&str) {
        match env::var(key) {
            Ok(value) => println!("'{}' : '{}'", key, value),
            Err(e) => println!("Couldn't read {}: {}", key, e),
        }
    }

    #[allow(dead_code)] 
    fn evar_delete(key:&str) {
        unsafe { env::remove_var(key); }
    }

    fn evar_add(key:&str, value:&str) {
        unsafe { env::set_var(key, value); }
    }

    fn run(args: Vec<&str>) -> Option<String> {

        match args.len() {
            0 => Evars::evar_print_all(),
            1 => Evars::evar_print_one(args[0]),
            2 => Evars::evar_add(args[0], args[1]),
            _ => return Some("Too many args for command 'evar'".to_owned())

        }

        None
    }

}

impl Executable for Evars{

    fn exec(args:Vec<&str>, localvars: &mut LocalVars) -> u8 {
        let _ = localvars;
        

        if let Some(err) = Evars::run(args){
            eprintln!("{}",err);
            return 1
        };

        0
    }

}