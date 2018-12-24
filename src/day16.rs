use regex::{Regex, Captures};
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn compute(input: &str) {
  let mut examples = vec![];
//  let initial_example = Example{before: [0,0,0,0], instruction: Instruction{op_code: 0, a: 0, b:0, c: 0}, after: [0,0,0,0]};
  let mut parts = input.split("\n\n\n\n");
  let example = parts.next().unwrap();
  let program = parts.next().unwrap();
  let regex = Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)] +(\d+) (\d+) (\d+) (\d+) +After: +\[(\d+), (\d+), (\d+), (\d+)]").unwrap();
  let mut example_iter = example.split("\n\n");
  // parse input
  while let Some(s) = example_iter.next() {
    let t = s.replace("\n","  ");
    let c = regex.captures(&t).expect("Should be able to match string");
    let u = |i: usize| -> usize {c.get(i).unwrap().as_str().trim().parse().unwrap()};
    let before = [u(1), u(2), u(3), u(4)];
    let op_code = u(5);
    let instruction = Instruction {
      a: u(6),
      b: u(7),
      c: u(8),
    };
    let after = [u(9), u(10), u(11), u(12)];
    let e = Example {before, op_code, instruction, after};
    examples.push(e);
  }
//  println!("Number of examples: {}", examples.len());

  // Figure out which opcodes possibly work for which numbers.
  let mut op_codes = vec![OpCode::addr, OpCode::addi, OpCode::mulr, OpCode::muli, OpCode::banr, OpCode::bani, OpCode::borr, OpCode::bori, OpCode::setr, OpCode::seti, OpCode::gtir, OpCode::gtri, OpCode::gtrr, OpCode::eqir, OpCode::eqri, OpCode::eqrr,];
  op_codes.sort();
  let mut op_code_map: Vec<HashSet<OpCode>> = vec![HashSet::from_iter(op_codes.clone().into_iter()); op_codes.len()];
  let mut triple_matches = 0;
  for (i, e) in examples.iter().enumerate() {
    let mut matches = 0;
    for (j, op) in op_codes.iter().enumerate() {
      if op.compute(&e.before, &e.instruction) == e.after {
        matches += 1;
      } else {
        op_code_map[e.op_code].remove(op);
      }
    }
    if matches >= 3 {
      triple_matches += 1;
    }
  }
  println!("Triple matches: {}", triple_matches);
//  println!("Op code map {:#?}", op_code_map);
  // Figure out which numbers are which opcodes
  for i in 0..op_codes.len() {
    let op_code_map_clone = op_code_map.clone();
    let singles: Vec<_> = op_code_map_clone.iter().filter(|v| v.len() == 1).collect();
    singles.iter().for_each(|v| {
      let op = v.iter().last().unwrap();
      op_code_map.iter_mut().for_each(|v| {
        if v.len() > 1 {
          v.remove(op);
        }
      })
    });
    if singles.len() == 16 {
      break;
    }
  }
//  println!("Op code map {:?}", op_code_map);
  let op_code_ids: Vec<OpCode> = op_code_map.into_iter().map(|s|s.into_iter().last().unwrap()).collect();
//  println!("Op code ids {:#?}", op_code_ids);
  let regex = Regex::new(r"(\d+) +(\d+) +(\d+) +(\d+)").unwrap();
  let mut register: Register = [0, 0, 0, 0];
  for line in program.lines() {
    let c = regex.captures(line).unwrap();
    let u = |i: usize| -> usize {c.get(i).unwrap().as_str().trim().parse().unwrap()};
    let op_code = &op_code_ids[u(1)];
    let i = Instruction{a: u(2), b: u(3), c: u(4)};
    register = op_code.compute(&register, &i);
  }
  println!("Register 0 = {}", register[0]);
}

fn to_usize(captures: Captures, place: usize) -> usize {
  captures.get(place).unwrap().as_str().trim().parse().unwrap()
}

#[derive(Debug, Clone, PartialEq)]
struct Example {
  before: Register,
  op_code: usize,
  instruction: Instruction,
  after: Register
}

type Register = [usize; 4];

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u32)]
enum OpCode {
  addr,
  addi,
  mulr,
  muli,
  banr,
  bani,
  borr,
  bori,
  setr,
  seti,
  gtir,
  gtri,
  gtrr,
  eqir,
  eqri,
  eqrr,
  unknown,
}

impl OpCode {
  fn compute(&self, input: &Register, i: &Instruction) -> Register {
    let mut ret = input.clone();
    match self {
      OpCode::addr => ret[i.c] = input[i.a] + input[i.b],
      OpCode::addi => ret[i.c] = input[i.a] + i.b,
      OpCode::mulr => ret[i.c] = input[i.a] * input[i.b],
      OpCode::muli => ret[i.c] = input[i.a] * i.b,
      OpCode::banr => ret[i.c] = input[i.a] & input[i.b],
      OpCode::bani => ret[i.c] = input[i.a] & i.b,
      OpCode::borr => ret[i.c] = input[i.a] | input[i.b],
      OpCode::bori => ret[i.c] = input[i.a] | i.b,
      OpCode::setr => ret[i.c] = input[i.a],
      OpCode::seti => ret[i.c] = i.a,
      OpCode::gtir => ret[i.c] = if i.a > input[i.b] {1} else {0},
      OpCode::gtri => ret[i.c] = if input[i.a] > i.b {1} else {0},
      OpCode::gtrr => ret[i.c] = if input[i.a] > input[i.b] {1} else {0},
      OpCode::eqir => ret[i.c] = if i.a == input[i.b] {1} else {0},
      OpCode::eqri => ret[i.c] = if input[i.a] == i.b {1} else {0},
      OpCode::eqrr => ret[i.c] = if input[i.a] == input[i.b] {1} else {0},
      _ => unimplemented!(),
    }
    ret
  }
}

#[derive(Debug, Clone, PartialEq)]
struct Instruction {
  a: usize,
  b: usize,
  c: usize,
}


