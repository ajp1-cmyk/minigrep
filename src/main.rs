use std::env;
use std::process;
use minigrep::Config;


fn main(){
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err|{
        println!("problem parsing arguments: {}",err);
        process::exit(1);
    });

    match minigrep::run(&config){
        
        Ok(()) => println!("success"),

        Err(err) => {
            println!("application error: {}",err);
            process::exit(1);
        }
    }
}
