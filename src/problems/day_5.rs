pub enum Part {
  One,
  Two,
}

pub fn part_1(data: String) -> String {
  run(data, Part::One)
    .to_string()
}

pub fn part_2(data: String) -> String {
  run(data, Part::Two)
    .to_string()
}

fn run(data: String, part: Part) -> i32 {
  let mut instructions = instructions(data);  
  let length = instructions.len();
  let mut index = 0;
  let mut steps = 0;
  let advance_formula = match part {
    Part::One => part_1_update,
    Part::Two => part_2_update,
  };
  while index < length as i32 {
    index = advance(&mut instructions, index as usize, &advance_formula);
    steps += 1;
    if index >= length as i32 { break }
  }
  steps
}

fn part_1_update(value: i32) -> i32 {
  value + 1
}

fn part_2_update(value: i32) -> i32 {
  if value >= 3 { value - 1 } else { value + 1 }
}


fn advance<F>(
  instructions: &mut Vec<i32>, 
  index: usize, 
  advance_formula: F) -> i32 where F: Fn(i32) -> i32 {
  let elem = instructions.get_mut(index).unwrap();
  let to_jump = *elem + (index as i32);
  *elem = advance_formula(*elem);
  to_jump
}

fn instructions(data: String) -> Vec<i32> {
  data
    .lines()
    .map(|l| l.parse().unwrap())
    .collect()
}
