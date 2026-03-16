use std::fs;
use std::error::Error;

pub struct Config {
    query: String,
    filename: String
}

impl Config{

    pub fn new(args: &[String]) -> Result<Config, &str> {
        
        if args.len() < 3{
            return Err("not enough arguments");
        } else if args.len() > 3 {
            return Err("too many arguments");
        }

        let query: String = args[1].clone();
        let filename: String = args[2].clone();   
        
        Ok(Config { query,filename})
        
    }

    pub fn query(&self) -> &String{
        &self.query
    }
    pub fn filename(&self) -> &String {
        &self.filename       
    }
}


pub fn run(config:&Config) -> Result<(),Box<dyn Error>>{
    println!("Search for: {}",config.query());
    println!("In File: {}",config.filename());

    let contents = fs::read_to_string(config.filename())?;
    println!("\n\nWith text:\n{}",contents);
    Ok(())
}

