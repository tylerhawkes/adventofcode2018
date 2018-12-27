use regex::Regex;

pub fn compute(input: &[String]) {
  let mut program = Program::from(input);
  println!("{:?}", program);
  let mut registers: Registers = [0, 0, 0, 0, 0, 0];
  program.run(&registers); // 6128512 instructions
//  registers[0] = 1;
//  program.run(&registers);
//  println!("instructions = {}", instructions_as_code(1));
//  println!("instructions = {}", instructions_as_fast_code(0));
  println!("instructions = {}", instructions_as_fast_code(1));
}

fn instructions_as_code(r0_start: usize) -> usize {
  let mut r0 = r0_start;
  let mut r1 = 0;
  let mut r3 = 0;
  let mut r4 = 0;
  let mut r5 = 0;
//  let mut r4 = 10551275;
  r4 += 2; //0,17
  r4 *= r4; //18
  r4 *= 19; //19
  r4 *= 11; //20
  r5 += 1; //21
  r5 *= 22; //22
  r5 += 17; //23
  r4 += r5; //24 // 875
  if r0 == 1 { //25,26
    r5 = 27; //27
    r5 *= 28; //28
    r5 += 29; //29
    r5 *= 30; //30
    r5 *= 14; //31
    r5 *= 32; //32
    r4 += r5; //33 // 10551275
    r0 = 0; //34
  } //35
  println!("{}, {}, {}, {}, {}", r0, r1, r3, r4, r5);
  r1 = 1; //1
  loop {
      r3 = 1; //2
    loop {
//      println!("{}, {}, {}, {}, {}", r0, r1, r3, r4, r5);
      r5 = r1 * r3; //3
      if r5 == r4 { //4,5,6
        r0 += r1; //7
      }
      r3 += 1; //8
      if r3 > r4 { //9
        break; //10
      }
    }//11
    r1 += 1; //12
    println!("{}, {}, {}, {}, {}", r0, r1, r3, r4, r5);
    if r1 > r4 { //13,14
      break; //16 //end program
    }
  }//15
  println!("{}, {}, {}, {}, {}", r0, r1, r3, r4, r5);
  r0
}

fn instructions_as_fast_code(r0_start: usize) -> usize {
  let mut r0 = r0_start;
  let mut r1 = 0;
  let mut r3 = 0;
  let mut r4 = 0;
  let mut r5 = 0;
//  let mut r4 = 10551275;
  r4 += 2; //0,17
  r4 *= r4; //18
  r4 *= 19; //19
  r4 *= 11; //20
  r5 += 1; //21
  r5 *= 22; //22
  r5 += 17; //23
  r4 += r5; //24 // 875
  if r0 == 1 { //25,26
    r5 = 27; //27
    r5 *= 28; //28
    r5 += 29; //29
    r5 *= 30; //30
    r5 *= 14; //31
    r5 *= 32; //32
    r4 += r5; //33 // 10551275
    r0 = 0; //34
  } //35
  let mut divisor_sum = 0;
  for i in 1..=r4 {
    if r4 % i == 0 {
      divisor_sum += i;
      if divisor_sum > r4 {
        return divisor_sum;
      }
    }
  }
  return 0;
}

#[derive(Debug)]
struct Program {
  instructions: Vec<(OpCode, Instruction)>,
  instruction_pointer_register: usize,
  instruction_pointer_value: usize,
}

impl Program {
  fn run(&mut self, start: &Registers) -> Registers {
    let mut registers = start.clone();
    println!("Starting with registers = {:?}", registers);
    let mut instruction_count = 0;
    while self.instruction_pointer_value < self.instructions.len() && registers[0] < 10 {
      // write instruction pointer value to register
      registers[self.instruction_pointer_register] = self.instruction_pointer_value;
      let instruction = &self.instructions[self.instruction_pointer_value];
//      println!("{:?} {}, {}, {}, {:?}", instruction.0, instruction.1.a, instruction.1.b, instruction.1.c, registers);
      registers = instruction.0.compute(&registers, &instruction.1);
      //increement instruction pointer value by one
      self.instruction_pointer_value = registers[self.instruction_pointer_register] + 1;
      instruction_count += 1;
    }
    println!("Registers at end of execution ({} instructions) is: {:?}", instruction_count, registers);
    self.instruction_pointer_value = 0;
    registers
  }
}

impl From<&[String]> for Program {
  fn from(input: &[String]) -> Self {
    let mut iter = input.iter();
    let ip_regex = Regex::new(r"#ip (\d+)").unwrap();
    let first = iter.next().expect(r"First line must match '#ip (\d+)'");
    let c = ip_regex.captures(first).expect("Unable to match first line");
    let instruction_pointer_register = c.get(1).unwrap().as_str().parse().unwrap();
    let mut instructions = Vec::with_capacity(input.len()-1);
    let instruction_regex = Regex::new(r"(\w+) (\d+) (\d+) (\d+)").unwrap();
    for line in iter {
      let c = instruction_regex.captures(line).unwrap();
      let u = |i: usize| -> usize {c.get(i).unwrap().as_str().trim().parse().unwrap()};
      let opcode = OpCode::from(c.get(1).unwrap().as_str());
      let instruction = Instruction {a: u(2), b: u(3), c: u(4)};
      instructions.push((opcode, instruction));
    }
    Program {instructions, instruction_pointer_register, instruction_pointer_value: 0}
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