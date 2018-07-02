use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_sensetive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)

        )

    }
    #[test]
    fn case_insensetive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensetive(query, contents)

        )

}
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }


    }
    results
}
pub fn search_case_insensetive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }


    }
    results
}


pub fn run(config: Config) -> Result<(), Box<Error>>{
    let mut f = File::open(config.filename)?;//.expect("File not found!");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let results = if config.case_sensetive {
        println!("sika");
        search(&config.query, &contents)
    } else {
        println!("suka!");
        search_case_insensetive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())


}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensetive: bool,

}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enogh args");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensetive = env::var("CASE_INSENSETIVE").is_err();

        Ok(Config { query, filename, case_sensetive } )
    }
}
