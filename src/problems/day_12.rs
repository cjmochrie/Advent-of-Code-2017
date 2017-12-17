use std::collections::HashSet;

type Programs =  Vec<Vec<usize>>;

pub fn part_1(data: String) -> String {
  let programs = data.lines()
    .map(|line| parse_line(line))
    .collect::<Programs>();

  // find connections of 0
  let mut visited: HashSet<usize> = HashSet::new();
  visited.insert(0);
  connections(&programs, 0, &mut visited);

  visited.len().to_string()
}

pub fn part_2(data: String) -> String {
  let programs = data.lines()
    .map(|line| parse_line(line))
    .collect::<Programs>();

  let mut groups = HashSet::new();
  for i in 0..programs.len() {
    let mut visited = HashSet::new();
    visited.insert(i);
    connections(&programs, i, &mut visited);
    let vec_visited = hashset_to_vec(visited);
    groups.insert(vec_visited);
  }

   groups.len().to_string()
}

fn hashset_to_vec(mut programs: HashSet<usize>) -> Vec<usize> {
  let mut vec = programs.drain().collect::<Vec<usize>>();
  vec.sort();
  vec
}

fn parse_line(data: &str) -> Vec<usize> {
  let mut links = data.split("<->");
  links.next();
  links.next().unwrap().trim().split(", ")
    .map(|num| num.parse().unwrap())
    .collect()
}

fn connections(programs: &Programs, program: usize, visited: &mut HashSet<usize>) {
  let to_visit = &programs[program];
  for connection in to_visit {
    if visited.insert(*connection) {
      connections(programs, *connection, visited);
    }
  }
}
