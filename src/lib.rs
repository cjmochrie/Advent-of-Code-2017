use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

fn day_1(_data: String) -> String {
  String::from("foo")
}

pub fn run(config: &Config) -> Result<String, Box<Error>> {
  let mut f = File::open(&config.filename)?;

  let mut contents = String::new();
  f.read_to_string(&mut contents)?;
  
  let result = match &config.problem[..] {
    "1" => day_1(contents),
     x =>  format!("Unknown problem {}", x),
  };
  Ok(result)
}

pub struct Config {
  pub problem: String,
  pub filename: String,
}

impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    args.next();

    let problem = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string"),
    };
      
    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a filename"),
    };
      
    Ok(Config { problem, filename })
  }
}

