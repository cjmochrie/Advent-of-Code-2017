use std::collections::HashSet;

pub fn part_1(data: String) -> String {
  let banks: Vec<u32> = parse_banks(data);
  count_cycles(banks).to_string()
}

pub fn part_2(data: String) -> String {
  let banks = parse_banks(data);
  count_infinite_repeat(banks).to_string()
}

fn parse_banks(data: String) -> Vec<u32> {
  data
    .split_whitespace()
    .map(|num| num.parse::<u32>())
    .filter_map(Result::ok)
    .collect()
}

fn count_infinite_repeat(mut banks: Vec<u32>) -> u32 {
  let length = banks.len();
  let mut patterns: HashSet<Vec<u32>> = HashSet::new();
  while patterns.insert(banks[..].to_owned()) {
    redistribute(&mut banks, length);
  }
  let pattern = banks[..].to_owned();

  redistribute(&mut banks, length);
  let mut cycles = 1;  
  
  while pattern != banks[..].to_owned(){
    redistribute(&mut banks, length);
    cycles += 1;
  }
  cycles
}

fn count_cycles(mut banks: Vec<u32>) -> u32 {
  let length = banks.len();
  let mut patterns: HashSet<Vec<u32>> = HashSet::new();
  let mut cycles = 0;
  while patterns.insert(banks[..].to_owned()) {
    redistribute(&mut banks, length);
    cycles += 1
  }
  cycles
}

fn redistribute(banks: &mut Vec<u32>, length: usize) {
  let max_index = find_max(&banks);
  let mut to_distribute = zero_out(banks, max_index);
  let mut index = max_index;
  while to_distribute > 0 {
    index = if index == length - 1 { 0 } else { index + 1 };
    let elem = banks.get_mut(index).unwrap();
    *elem += 1;
    to_distribute -= 1;
  }
} 

/// zero out the max bank and return the balance to be redistributed
fn zero_out(banks: &mut Vec<u32>, index: usize) -> u32 {
  let balance = banks.get_mut(index).unwrap();
  let value = balance.clone();
  *balance = 0;
  value
}

/// Find index of the max value in the bank
fn find_max(banks: &Vec<u32>) -> usize {
  banks.clone().iter().enumerate()
    .fold((0, 0), |(prev_index, prev_max), (index, &max)|
      if max > prev_max { (index, max) } else { (prev_index, prev_max) }
    ).0
}
