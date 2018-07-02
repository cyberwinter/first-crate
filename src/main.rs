extern crate first_crate;

use std::env;
use std::process;

use first_crate::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {

        process::exit(1);
    });

    if let Err(e) = first_crate::run(config) {

        process::exit(1);
    }

}
