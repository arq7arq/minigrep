use std::error::Error;
use std::io::Read;
use std::{fs, io};
pub struct Config{
    pub query: String,
    pub filename: Option<String>,
    pub case_sensitive: bool
}

impl Config {

    pub fn new(args: &[String]) -> Result<Config, &str>{


        let mut argumets = Vec::new();

        let mut case_sensitive: bool = true;

        for arg in args.iter().skip(1){
            match arg.as_str(){
                "-i" => {
                    case_sensitive = false;
                }
                _ => {
                    argumets.push(arg);
                }
            }
        }

        if argumets.is_empty(){
            return  Err("Missing query");
        }

        let query= argumets[0].clone();
        let filename= argumets.get(1).map(|value| (*value).clone());

        Ok(Config { query, filename , case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = match config.filename {
        
        Some(value) => fs::read_to_string(value)?,
        None => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            input
        }
    };

    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    }
    else{
        search_case_insensitive(&config.query, &contents)
    };

    for line in results{
        println!("{}",line);
    }


    return Ok(());
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
mod tests {
    use super::*;

    #[test]
    fn case_sensitive(){

        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Ductape.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));

    }

    #[test]
    fn case_insentivie(){

        let query = "rUST";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:","Trust me."], search_case_insensitive(query, contents));

    }
}