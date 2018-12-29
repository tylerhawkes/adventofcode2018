use regex::Regex;
use std::collections::{HashMap, hash_map::Entry};

pub fn compute(input: &[String]) {
  let mut program = Program::from(input);
  let mut registers: Registers = [0, 0, 0, 0, 0, 0];
  registers[0] = 15615244;
  program.run(&registers);
}

#[derive(Debug)]
struct Program {
  instructions: Vec<(OpCode, Instruction)>,
  instruction_pointer_register: usize,
  instruction_pointer_value: usize,
}

impl Program {
  fn run(&mut self, start: &Registers) -> usize {
    let mut registers = start.clone();
    println!("Starting with registers = {:?}", registers);
    let mut halting_order = Vec::with_capacity(16384);
    let mut halting_values: HashMap<u32, usize> = HashMap::new();
    let mut instruction_count = 0;
    while self.instruction_pointer_value < self.instructions.len() {
      // write instruction pointer value to register
      registers[self.instruction_pointer_register] = self.instruction_pointer_value;
      let instruction = &self.instructions[self.instruction_pointer_value];
//      println!("{:?} {}, {}, {}, {:?}", instruction.0, instruction.1.a, instruction.1.b, instruction.1.c, registers);
      registers = instruction.0.compute(&registers, &instruction.1);
      if instruction.0 == OpCode::prnt {
        match halting_values.entry(registers[5] as u32) {
          Entry::Vacant(v) => {
            v.insert(instruction_count);
            halting_order.push(registers[5] as u32);
          },
          Entry::Occupied(_) => {
            break;
          },
        }
      }
      //increement instruction pointer value by one
      self.instruction_pointer_value = registers[self.instruction_pointer_register] + 1;
      instruction_count += 1;
    }
//    println!("{}\n{:?}", halting_order.len(), halting_order);
    println!("Registers at end of execution ({} instructions) is: {:?}", instruction_count, registers);
    println!("Min: {}, Max: {}", halting_order[0], halting_order[halting_order.len()-1]);
    self.instruction_pointer_value = 0;
    instruction_count
  }
}

impl From<&[String]> for Program {
  fn from(input: &[String]) -> Self {
    let mut iter = input.iter();
    let ip_regex = Regex::new(r"#ip (\d+)").unwrap();
    let first = iter.next().expect(r"First line must match '#ip (\d+)'");
    let c = ip_regex.captures(first).expect("Unable to match first line");
    let instruction_pointer_register = c.get(1).unwrap().as_str().parse().unwrap();
    let mut instructions = Vec::with_capacity(input.len() - 1);
    let instruction_regex = Regex::new(r"(\w+) (\d+) (\d+) (\d+)").unwrap();
    for line in iter {
      let c = instruction_regex.captures(line).unwrap();
      let u = |i: usize| -> usize { c.get(i).unwrap().as_str().trim().parse().unwrap() };
      let opcode = OpCode::from(c.get(1).unwrap().as_str());
      let instruction = Instruction { a: u(2), b: u(3), c: u(4) };
      instructions.push((opcode, instruction));
    }
    Program { instructions, instruction_pointer_register, instruction_pointer_value: 0}
  }
}

type Registers = [usize; 6];

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
  prnt,
  unknown,
}

impl OpCode {
  fn compute(&self, input: &Registers, i: &Instruction) -> Registers {
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
      OpCode::gtir => ret[i.c] = if i.a > input[i.b] { 1 } else { 0 },
      OpCode::gtri => ret[i.c] = if input[i.a] > i.b { 1 } else { 0 },
      OpCode::gtrr => ret[i.c] = if input[i.a] > input[i.b] { 1 } else { 0 },
      OpCode::eqir => ret[i.c] = if i.a == input[i.b] { 1 } else { 0 },
      OpCode::eqri => ret[i.c] = if input[i.a] == i.b { 1 } else { 0 },
      OpCode::eqrr => ret[i.c] = if input[i.a] == input[i.b] { 1 } else { 0 },
      OpCode::prnt => {
//        println!("r.{} = {}", i.a, input[i.a]);
        ret[i.c] = i.b;
      },
      _ => unimplemented!(),
    }
    ret
  }
}

impl From<&str> for OpCode {
  fn from(input: &str) -> Self {
    match input {
      "addr" => OpCode::addr,
      "addi" => OpCode::addi,
      "mulr" => OpCode::mulr,
      "muli" => OpCode::muli,
      "banr" => OpCode::banr,
      "bani" => OpCode::bani,
      "borr" => OpCode::borr,
      "bori" => OpCode::bori,
      "setr" => OpCode::setr,
      "seti" => OpCode::seti,
      "gtir" => OpCode::gtir,
      "gtri" => OpCode::gtri,
      "gtrr" => OpCode::gtrr,
      "eqir" => OpCode::eqir,
      "eqri" => OpCode::eqri,
      "eqrr" => OpCode::eqrr,
      "prnt" => OpCode::prnt,
      _ => OpCode::unknown,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
struct Instruction {
  a: usize,
  b: usize,
  c: usize,
}