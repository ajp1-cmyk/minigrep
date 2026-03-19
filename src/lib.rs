use std::fs;
use std::error::Error;
use colored::Colorize;
use std::env;


pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
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
        let case_sensitive = match env::var("CASE_SENSITIVE"){
            Err(_) => false,
            Ok(flag) => flag.parse().unwrap_or(false)
        };
        
        Ok(Config { query,filename, case_sensitive})
        
    }

    pub fn query(&self) -> &String{
        &self.query
    }
    pub fn filename(&self) -> &String {
        &self.filename       
    }

    pub fn case_sensitive(&self) -> &bool{
        &self.case_sensitive
    }
}


pub fn run(config:&Config) -> Result<(),Box<dyn Error>>{
    println!("\nSearch for: {}",config.query().bright_yellow());
    println!("In File: {}\n",config.filename().bright_red());

    let contents = fs::read_to_string(config.filename())?;
    
    
    // choosing option -> case sensitive or insensitive
    let result = if config.case_sensitive{
        search(config.query(), &contents)
    } else {
        search_case_insensitive(config.query(), &contents)
    };
    pretty_print(result,config.query());

    Ok(())
}

fn pretty_print(result: Vec<&str>, query:&str) {
        
    for line in result{
    
        for word in line.replace("\n", " \n").split(" "){
            if word.contains(query){
                print!("{} ",word.blue().italic().bold());
            }else {
                print!("{} ",word);
            }
        }
        println!("");
    }

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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();

    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results

}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["Rust:"], search_case_insensitive(query, contents));
    }



    #[test]
    fn case_sensitive(){
        let query = "Duct";
          let contents = "\
            Rust:
safe, fast, productive.
Pick three,
Duct tape.";
            println!("{}",contents);
        assert_eq!(vec!["Duct tape."], search(query,contents));
    }
}
