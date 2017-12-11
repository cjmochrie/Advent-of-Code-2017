extern crate advent_of_code;

use std::env;
use std::process;

use advent_of_code::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match advent_of_code::run(&config) {
        Err(e) => { 
            eprintln!("Application error: {}", e);
            process::exit(1);
        },
        Ok(result) => {
            println!("Result of Day {} was: {}", config.problem, result);
        }
    }
}
