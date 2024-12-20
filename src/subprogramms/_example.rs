use crate::SExecutable;

pub struct Example { }

impl Example {
    
    fn run(&mut self, args:Vec<&str>) -> Option<String> {
        
        None
    }

}

impl SExecutable for Example {
    
    fn exec(&mut self, args:Vec<&str>) -> u8 {

        if let Some(err) = self.run(args){
            eprintln!("{}",err);
            return 1
        };

        0
    }

}