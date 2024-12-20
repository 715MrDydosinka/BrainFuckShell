use std::env;

use crate::Executable;
use crate::LocalVars;

use super::evar::Evars;

pub struct CD { }

impl CD {
    
    fn cd(path: &str) -> Option<String> {

        let current_dir = env::current_dir().unwrap();

        let new_dir = if path == ".." {
            current_dir.parent().unwrap_or_else(|| &current_dir).to_path_buf()
        } else {
            let new_dir = current_dir.join(path);
            if new_dir.exists() && new_dir.is_dir() {
                new_dir
            } else {
                return Some("Directory not found".to_owned());
            }
        };
 
        if env::set_current_dir(&new_dir).is_err() {
            return Some("Failed to change directory".to_owned())
        };

        return None
    }

    fn cd_home() -> Option<String> {
        
        match Evars::evar_get_one("HOME") {
            Ok(value) => CD::cd(&value),
            Err(e) => { return Some(e.to_string()) }
        };
        
        None
    }


    fn run(args:Vec<&str>) -> Option<String> {
        
        match args.len() {
            0 => CD::cd_home(),
            1 => CD::cd(args[0]),
            _ => return Some("Too many args for command 'cd'".to_owned())
        };
        
        None
    }

}

impl Executable for CD {
    
    fn exec(args:Vec<&str>, localvars: &mut LocalVars) -> u8 {
        let _ = localvars;

        if let Some(err) = CD::run(args){
            eprintln!("{}",err);
            return 1
        };

        0
    }

}