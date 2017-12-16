const LIST_LENGTH: u32 = 256;
const ROUNDS: u32 = 64;

#[derive(Debug)]
struct State {
  position: u32,
  skip_size: u32,
  list: Vec<u32>,
}

impl State {
  pub fn new() -> State {
    State { position: 0, skip_size: 0, list: (0..LIST_LENGTH).collect() }
  }

  pub fn advance(&mut self, length: u32) {
    self.list = State::reverse_list(self.position, length, &self.list);
    self.position = (self.position + length + self.skip_size) % LIST_LENGTH;
    self.skip_size += 1;
  }

  fn reverse_list(position: u32, length: u32, list: &Vec<u32>) -> Vec<u32> {
    let mut sub_list = vec![];

    // extract the part of the list that will be reversed and reverse it
    for index in position..(position + length) {
      sub_list.push(list[index as usize % LIST_LENGTH as usize]);
    }
    sub_list.reverse();
    
    // add the remaining elements in the correct order
    for index in (position + length)..(position + LIST_LENGTH) {
      sub_list.push(list[index as usize % LIST_LENGTH as usize]);
    }

    // Re-orient the list in its original positions
    let mut final_list = vec![0; LIST_LENGTH as usize];
    for (index, element) in sub_list .into_iter().enumerate() {
      final_list[(position as usize + index) % LIST_LENGTH as usize] = element;
    }
    final_list
  }

}

pub fn part_1(data: String) -> String {
  let mut state = State::new();
  for length in read_lengths(data).into_iter() {
    state.advance(length);
  }
  (state.list[0] as u32 * state.list[1] as u32).to_string()
}

pub fn part_2(data: String) -> String {
  let mut data = to_ascii(data);
  data.append(&mut vec![17, 31, 73, 47, 23]);
  let result = hash_rounds(data);
  let result = to_dense_hash(result);
  to_hex_string(result)
}

fn to_hex_string(bytes: Vec<u8>) -> String {
  bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn hash_rounds(lengths: Vec<u32>) -> Vec<u32> {
  let mut state = State::new();
  for _ in 0..ROUNDS {
    for length in lengths.iter() {
      state.advance(*length);
    }
  }
  state.list
}

fn to_dense_hash(data: Vec<u32>) -> Vec<u8> {
  let mut result = vec![];
  for chunk in data[..].chunks(16) {
    let dense = chunk.iter().fold(0, |accum, c| accum as u8 ^ *c as u8);
    result.push(dense);
  }
  result
}

fn to_ascii(data: String) -> Vec<u32> {
  data.trim().chars().map(|c| c as u32).collect()
}

fn read_lengths(data: String) -> Vec<u32> {
  data.trim().split(",")
    .map(|num| num.parse::<u32>().unwrap())
    .collect()
}

