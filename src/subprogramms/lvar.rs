use crate::local_vars::GetValue;
use crate::local_vars::SetValue;
use crate::Executable;
use crate::LocalVars;

pub struct Lvars { }

impl Lvars {

    fn lvar_print_all(localvars: &LocalVars) {

        let hui = localvars.store.clone();

        if hui.is_empty() {
            println!("[Empty]");
            return
        }
        for (key, value) in &hui{
            println!("'{}' : '{}'", key, value)
        }
 
    }

    fn lvar_print_one(localvars: &LocalVars, key:&str) {
        if let Some(value) = localvars.get_str(key) {
            println!("'{}' : '{}'", key, value);
        } else {
            println!("Variable '{}' is missing.", key);
        }
 
    }

    #[allow(dead_code)] 
    fn lvar_delete(localvars: &mut LocalVars ,key:&str) {
        let _ = key;
        let _ = localvars;    
    }

    fn lvar_add(localvars: &mut LocalVars, key:&str, value:&str) {
        localvars.set_str(key, value);
    }

    fn run(args: Vec<&str>, localvar: &mut LocalVars) -> Option<String> {

        match args.len() {
            0 => Lvars::lvar_print_all(&localvar),
            1 => Lvars::lvar_print_one(&localvar, args[0]),
            2 => Lvars::lvar_add(localvar, args[0], args[1]),
            _ => return Some("Too many args for command 'ivar'".to_owned())

        }

        None
    }

}

impl Executable for Lvars{

    fn exec(args:Vec<&str>, localvars: &mut LocalVars) -> u8 {
        let _ = localvars;
        

        if let Some(err) = Lvars::run(args, localvars){
            eprintln!("{}",err);
            return 1
        };

        0
    }

}