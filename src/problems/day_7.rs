use regex::Regex;
use std::collections::{HashMap,HashSet};

#[derive(Debug)]
struct Program <'a> {
  name: &'a str,
  weight: i32,
  children: Vec<&'a str>,
}


impl <'a> Program<'a> {
  pub fn new(data: &str) -> Program {
    let re = Regex::new(r"(?x)
    (?P<name>\w+)
    \s
    \((?P<weight>\d+)\)
    (\s->\s)?
    (?P<children>
      [(\w+),\s?]+
    )?").unwrap();
    let cap = re.captures_iter(data).next().unwrap();
    let name = cap.name("name").unwrap().as_str();
    let weight = cap.name("weight").unwrap().as_str().parse().unwrap();
    let children: Vec<&str> = match cap.name("children") {
      Some(names) => names.as_str().split(", ").collect(),
      None => vec![],
    };
    Program { name: name, weight: weight, children: children }
  }
}

pub fn part_2(data: String) -> String {
  let root = part_1(data.clone());
  let mut programs: HashMap<&str, Program> = HashMap::new();
  for line in data.lines() {
    let program = Program::new(line);
    programs.insert(program.name, program);     
  }
  odd_one_out(&programs, &root).to_string()
}

pub fn part_1(data: String) -> String {
  let mut programs: HashMap<&str, Program> = HashMap::new();
  let mut children: HashSet<&str> = HashSet::new();
  let mut names: HashSet<&str> = HashSet::new();
  for line in data.lines() {
    let program = Program::new(line);
    names.insert(program.name);   
    for child in program.children[..].iter() {
      children.insert(child);
    }
    programs.insert(program.name, program);     
  }
  names.difference(&children).next().unwrap().to_string()
}

fn odd_one_out<'a>(programs: &'a HashMap<&str, Program>, parent: &'a str) -> i32 {
  let program = &programs.get(parent).unwrap();
  let mut weights: HashMap<i32, Vec<&str>> = HashMap::new();
  for child in program.children.iter() {
    let child_program = &programs.get(child).unwrap();
    let program_weight = child_weight(programs, &child_program.children) + child_program.weight;
    (*weights.entry(program_weight).or_insert(vec![])).push(child);
  }
  
  let mut child_and_weights: Vec<(i32, Vec<&str>)> = weights.iter()
    .map(|(weight, children)| (*weight, children[..].to_owned()))
    .collect();
  child_and_weights.sort_by(|a, b| (a.1.len()).cmp(&b.1.len()));
  
  let odd_child = child_and_weights[0].1[0];
  let odd_child_program = &programs.get(odd_child).unwrap();
  
  if children_balanced(programs, odd_child) {
    // It's this child's fault
    let expected_weight = child_and_weights[1].0;
    expected_weight - child_weight(programs, &odd_child_program.children)
  } else {
    // it's one of the child's children
    odd_one_out(programs, odd_child)
  }
}

fn children_balanced<'a>(programs: &'a HashMap<&str, Program>, parent: &'a str) -> bool {
  let mut weights: HashSet<i32> = HashSet::new();
  for child in programs.get(parent).unwrap().children.iter() {
    let program = programs.get(child).unwrap();
    let weight = program.weight + child_weight(programs, &program.children);
    weights.insert(weight);
  }
  weights.len() == 1
}

//If children all the same weight I am out of order, report my weight
//If children not the same weight, report what they say is out of order
// weight of other children - weight of child's children = what it should b

fn child_weight(programs: &HashMap<&str, Program>, children: &Vec<&str>) -> i32 {
  children[..].iter().fold(0, |sum, child| {
    let child_program = &programs.get(child).unwrap();
    sum + child_program.weight + child_weight(programs, &child_program.children)
  })
}

