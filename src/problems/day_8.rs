use std::collections::HashMap;

#[derive(Debug)]
struct Instruction<'a> {
  register: &'a str,
  op: Op,
  op_amount: i32,
  condition_register: &'a str,
  condition: Condition,
  condition_value: i32,
}

#[derive(Debug)]
struct Register {
 value: i32,
 highest: i32, 
}

impl Register {
  pub fn new() -> Register {
    Register { value: 0, highest: 0 }
  }
}

#[derive(Debug)]
enum Condition {
  Lt,
  Gt,
  Lte,
  Gte,
  Eql,
  Ne
}

#[derive(Debug)]
enum Op {
  Inc,
  Dec,
}

impl <'a> Instruction<'a> {
  pub fn new(line: &str) -> Instruction {
    let mut parts = line.split_whitespace();
    let name = parts.next().unwrap();
    let op = match parts.next().unwrap() {
      "inc" => Op::Inc,
      "dec" => Op::Dec,
      _ => panic!(),
    };
    let op_amount = parts.next().unwrap().parse();
    parts.next();
    let condition_register = parts.next().unwrap();
    let condition = match parts.next().unwrap() {
      "<" => Condition::Lt,
      ">" => Condition::Gt,
      "<=" => Condition::Lte,
      ">=" => Condition::Gte,
      "==" => Condition::Eql,
      "!=" => Condition::Ne,
      _ => panic!(),
    };
    let condition_value = parts.next().unwrap().parse();

    Instruction {
      register: name,
      op: op,
      op_amount: op_amount.expect("An int"),
      condition_register: condition_register,
      condition: condition,
      condition_value: condition_value.expect("An int"),
    }
  }
}

pub fn part_1(data: String) -> String {
  let instructions: Vec<Instruction> = data.lines().map(|line| Instruction::new(line)).collect();
  let result = execute_instructions(instructions);
  find_largest_register_value(result).to_string()
}

pub fn part_2(data: String) -> String {
  let instructions: Vec<Instruction> = data.lines().map(|line| Instruction::new(line)).collect();
  let result = execute_instructions(instructions);
  find_largest_register_value_ever(result).to_string()
}

fn execute_instructions(instructions: Vec<Instruction>) -> HashMap<&str, Register> {
  let mut registers: HashMap<&str, Register> = HashMap::new();
  for instruction in instructions {
    compute_instruction(&mut registers, instruction);
  }
  registers
}

fn compute_instruction <'a>(
  registers: &mut HashMap<&'a str, Register>,
   instruction: Instruction<'a>) {
  use self::Op;
  
  let condition_register_value = get_register_value(registers, instruction.condition_register);
  if satisfies_condition(
    instruction.condition, 
    condition_register_value, 
    instruction.condition_value) {
    let register_change = match instruction.op {
      Op::Inc => instruction.op_amount,
      Op::Dec => -instruction.op_amount,
    };
    let register = registers.entry(instruction.register).or_insert(Register::new());
    register.value += register_change;
    if register_change + register.value > register.highest {
      register.highest = register.value;
    }
  }

}

fn get_register_value(registers: &HashMap<&str, Register>, name: &str) -> i32 {
  match registers.get(name) {
    Some(register) => register.value,
    None => 0,
  }
}

fn satisfies_condition(condition: Condition, a: i32, b: i32) -> bool {
  use self::Condition::*;
  match condition {
    Lt => a < b,
    Gt => a > b,
    Lte => a <= b,
    Gte => a >= b,
    Eql => a == b,
    Ne => a != b,
  }
}

fn find_largest_register_value(registers: HashMap<&str, Register>) -> i32 {
  let mut results: Vec<(&&str, &Register)> = registers.iter()
    .collect();
  results.sort_by(|a, b| (b.1.value).cmp(&a.1.value));
  results[0].1.value
}

fn find_largest_register_value_ever(registers: HashMap<&str, Register>) -> i32 {
  let mut results: Vec<(&&str, &Register)> = registers.iter()
    .collect();
  results.sort_by(|a, b| (b.1.highest).cmp(&a.1.highest));
  results[0].1.highest
}
