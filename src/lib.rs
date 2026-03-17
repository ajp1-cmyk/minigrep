use std::fs;
use std::error::Error;
use colored::Colorize;

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
    println!("\nSearch for: {}",config.query().bright_yellow());
    println!("In File: {}\n",config.filename().bright_red());

    let contents = fs::read_to_string(config.filename())?;

    
    for line in search(config.query(), &contents){
    
        for word in line.replace("\n", " \n").split(" "){
            if word.contains(config.query()){
                print!("{} ",word.blue().italic().bold());
            }else {
                print!("{} ",word);
            }
        }
        println!("");
    }

    Ok(())
}

pub fn search<'a>(query:&str, contents:&'a str)-> Vec<&'a str>{
    
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
          let contents = "\
            Rust:
safe, fast, productive.
Pick three.";
            println!("{}",contents);
        assert_eq!(vec!["safe, fast, productive."], search(query,contents));
    }
}
