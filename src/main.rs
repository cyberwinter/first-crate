extern crate first_crate;

use std::env;
use std::process;

use first_crate::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    if let Err(e) = first_crate::run(config) {
        println!("AppErrro!{}", e);
        process::exit(1);
    }

}
