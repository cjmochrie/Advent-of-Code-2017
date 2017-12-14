use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

mod utils;
mod problems;
use problems::*;

pub fn run(config: &Config) -> Result<String, Box<Error>> {
  let mut f = File::open(&config.filename)?;

  let mut contents = String::new();
  f.read_to_string(&mut contents)?;
  
  let result = match &config.problem[..] {
    "1" => day_1::part_1(contents),
    "1.2" => day_1::part_2(contents),
    "2" => day_2::part_1(contents),
    "2.2" => day_2::part_2(contents),
    "3" => day_3::part_1(contents),
    "3.2" => day_3::part_2(contents),
    "4" => day_4::part_1(contents),
    "4.2" => day_4::part_2(contents),
    "5" => day_5::part_1(contents),
    "5.2" => day_5::part_2(contents),
    "6" => day_6::part_1(contents),
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

