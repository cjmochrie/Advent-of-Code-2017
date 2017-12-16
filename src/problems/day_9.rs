#[derive(Debug, PartialEq, Clone)]
enum Symbol {
  Open,
  Close,
  OpenGarbage,
  CloseGarbage,
  Bang,
  Other
}

pub fn part_1(data: String) -> String {
  let symbols = read_data(data);
  let symbols = strip_bangs(symbols);
  score_stream(symbols).to_string()
}

pub fn part_2(data: String) -> String {
  let symbols = read_data(data);
  let symbols = strip_bangs(symbols);
  count_garbage(symbols).to_string()
}

fn count_garbage(symbols: Vec<Symbol>) -> i32 {
  use self::Symbol::*;

  let mut current_garbage = 0;
  let mut garbage = false;
  for symbol in symbols.into_iter() {
    match symbol {
      OpenGarbage => {
        if garbage {
          current_garbage += 1;
        } else { 
          garbage = true;
        }
      },
      CloseGarbage => {
        garbage = false;
      },
      _ => { 
        if garbage { current_garbage += 1; }
      },
    }
  }
  current_garbage
}

fn score_stream(symbols: Vec<Symbol>) -> i32 {
  use self::Symbol::*;

  let mut current_score = 0;
  let mut open_groups = 0;
  let mut garbage = false;
  for symbol in symbols.into_iter() {
    match symbol {
      Open => {
        if garbage { continue }
        open_groups += 1;
      },
      Close => {
        if garbage { continue }
        current_score += open_groups;
        open_groups -= 1;
      },
      OpenGarbage => {
        garbage = true;
      },
      CloseGarbage => {
        garbage = false;
      },
      _ => {},
    }
  }
  current_score
}
fn read_data<'a>(data: String) -> Vec<Symbol> {
  use self::Symbol::*;

  Box::new(data.chars().map(|c|
    match c {
      '{' => Open,
      '}' => Close,
      '<' => OpenGarbage,
      '>' => CloseGarbage,
      '!' => Bang,
      _ => Other,
    })).collect()
}

fn strip_bangs(symbols: Vec<Symbol>) -> Vec<Symbol> {
  use self::Symbol::*;

  let mut stripped = vec![];
  let mut symbols = symbols.into_iter();
  loop {
    match symbols.next() {
      Some(symbol) => {
        if symbol == Bang {
          symbols.next();
        } else {
          stripped.push(symbol);
        }
      },
      None => { break },
    }
  }
  stripped
}
