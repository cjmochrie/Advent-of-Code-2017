use std::collections::HashMap;
use std::cmp;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Direction {
  N,
  NE,
  SE,
  S,
  SW,
  NW,
}

type DirectionCounter = HashMap<Direction, u32>;

pub fn new_counter() -> DirectionCounter {
  use self::Direction::*;

  let mut counter = HashMap::new();
  for direction in vec![N, NE, SE, S, SW, NW].into_iter() {
    counter.insert(direction, 0 as u32);
  }
  counter
}

pub fn part_1(data: String) -> String {
  let directions = parse_directions(data);
  let counter = count_directions(directions);
  let counter = redundant_opposites(counter);  
  let counter = simplify(counter);
  count_steps(counter).to_string()
}

// lol compiling with --release :sunglasses_emoji
pub fn part_2(data: String) -> String {
  let mut max_steps = 0;
  let directions = parse_directions(data);
  for i in 0..directions.len() {
    let steps = directions[0..i].to_owned();
    let counter = count_directions(steps);
    let counter = redundant_opposites(counter);  
    let counter = simplify(counter);
    let num_steps = count_steps(counter);
    max_steps = cmp::max(num_steps, max_steps);
  }
  max_steps.to_string()
}

fn count_steps(counter: DirectionCounter) -> u32 {
  counter.values().sum()
}

fn parse_directions(data: String) -> Vec<Direction> {
  use self::Direction::*;

  data.trim().split(",")
    .map(|d| match d {
      "n" => N,
      "ne" => NE,
      "se" => SE,
      "s" => S,
      "sw" => SW,
      "nw" => NW,
      _ => panic!("Unrecognized direction"),
    })
    .collect()
}



fn redundant_opposites(counter: DirectionCounter) -> DirectionCounter {
  use self::Direction::*;
  let opposites = vec![(N, S), (NE, SW), (SE, NW)];
  let mut new_counter: DirectionCounter = new_counter();
  for (a, b) in opposites.into_iter() {
    let a_count = counter.get(&a).unwrap();
    let b_count = counter.get(&b).unwrap();
    if a_count >= b_count {
      new_counter.insert(a, a_count - b_count);
    } else {
      new_counter.insert(b, b_count - a_count);
    }
  }
  new_counter
}

fn simplify(counter: DirectionCounter) -> DirectionCounter {
  use self::Direction::*;
  let simplifications = vec![
      ((N, SE), NE),
      ((NE, S), SE),
      ((SE, SW), S),
      ((S, NW), SW),      
      ((SW, N), NW),
      ((NW, NE), N),
    ];
  let mut new_counter = counter.clone();
  for ((a, b), result) in simplifications.into_iter() {
    simplify_pair(a, b, result, &mut new_counter);
  }
  new_counter
}

fn simplify_pair(a: Direction, b: Direction, result: Direction, counter: &mut DirectionCounter) {
  let a_count = counter.get(&a).unwrap().clone();
  let b_count = counter.get(&b).unwrap().clone();
  let min_a_b = cmp::min(a_count, b_count);
  let result_count = counter.get(&result).unwrap().clone();
  if min_a_b > 0 {
    let new_result = result_count + min_a_b;
    counter.insert(a, a_count - min_a_b);
    counter.insert(b, b_count - min_a_b);
    counter.insert(result, new_result);
  }
}

fn count_directions(directions: Vec<Direction>) -> DirectionCounter {
  let mut counter = new_counter();
  for direction in directions {
    let entry = counter.get_mut(&direction).unwrap();
    *entry += 1;
  }
  counter
}
