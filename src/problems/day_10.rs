const LIST_LENGTH: u32 = 256;

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
    for index in position..(position + length) {
      sub_list.push(list[index as usize % LIST_LENGTH as usize]);
    }
    sub_list.reverse();
    
    for index in (position + length)..(position + LIST_LENGTH) {
      sub_list.push(list[index as usize % LIST_LENGTH as usize]);
    }
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
  String::from("foo")
}

fn read_lengths(data: String) -> Vec<u32> {
  data.trim().split(",")
    .map(|num| num.parse::<u32>().unwrap())
    .collect()
}

