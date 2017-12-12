use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Direction {
  Right,
  Up,
  Left,
  Down,
}

#[derive(Debug)]
pub struct State {
  direction: Direction,
  pub steps: u32,
  row: i32,
  column: i32,
  size: i32,
  board: Board,
}

type Board = HashMap<(i32, i32), i32>; 

impl State {
  pub fn take_step(&mut self) {
    self.advance();    
    if self.time_to_grow() {
      self.size += 2;
    }
    if self.check_corner() {
      self.turn();
    }
    let square_value = self.current_square_value();
    self.board.insert((self.row, self.column), square_value);
  }

  pub fn current_square_value(&self) -> i32 {
    let mut values = vec![];
    for i in (self.row - 1)..(self.row + 2) {
      for j in (self.column - 1)..(self.column + 2) {
        if i == self.row && j == self.column { continue; }
        values.push(self.board.get(&(i, j)));
      }
    }
    values.into_iter().filter_map(|x| x).sum()
  }

  fn time_to_grow(&self) -> bool {
    (self.steps as f64).sqrt() as i32 == self.size
  }
  pub fn new() -> State {
    let mut board = Board::new();
    board.insert((0, 0), 1);
    State { direction: Direction::Right, steps: 1, row: 0, column: 0, size: 3, board: board }
  }

  fn turn(&mut self) {
    use self::Direction::*;
    self.direction = match self.direction {
      Right => Up,
      Up => Left,
      Left => Down,
      Down => Right,
    };
  }

  fn advance(&mut self) {
    use self::Direction::*;
    
    match self.direction {
      Right => { self.column += 1; },
      Up => { self.row += 1; },
      Left => { self.column -= 1; },
      Down => { self.row -= 1; },
    };
    self.steps += 1;    
  }

  fn check_corner(&self) -> bool {
    use self::Direction::*;

    match self.direction {
      Right => self.column == self.size / 2,
      Up => self.row == self.size / 2,
      Left => -self.column == self.size / 2,
      Down => -self.row == self.size / 2,
    }
  }

}


pub fn part_1(data: String) -> String {
  let steps = data.lines().next().unwrap().parse::<u32>().unwrap();
  let (row, column) = walk(steps);
  distance(row, column).to_string()
}

pub fn part_2(data: String) -> String {
  let breakpoint = data.lines().next().unwrap().parse::<i32>().unwrap();
  walk_until(breakpoint).to_string()
}

fn walk_until(breakpoint: i32) -> i32 {
  let mut state = State::new();
  while state.current_square_value() < breakpoint {
    state.take_step();
  }
  state.current_square_value()
}

fn walk(steps: u32) -> (i32, i32) {
  let mut state = State::new();
  for _ in 1..steps {
    state.take_step();
  }
  (state.row, state.column)
}

fn distance(row: i32, column: i32) -> i32 {
  row.abs() + column.abs()
}
