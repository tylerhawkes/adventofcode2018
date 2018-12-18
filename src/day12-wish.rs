use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Display;

pub fn compute(input: &[String]) {
  let left_5: u64 = 0x1f << 59;
  println!("{:x}", left_5);
  let mut iter = input.iter();
  let initial_state = (&iter.next().unwrap().clone()[15..]).to_owned();
  let initial_state: Vec<bool> = initial_state.chars().map(|c| c == '#').collect();
  iter.next();
  let mut true_states = Vec::<u64>::with_capacity(iter.size_hint().0);
  for rule in iter {
    let k = &rule[0..5];
    let key: u64 = k.chars().rev().enumerate().map(|(i, c)| if c == '#' { 1 << (i) } else { 0 }).sum();
    let v = &rule[rule.len() - 1..];
    let v = v == "#";
    if v {
      true_states.push(key);
    }
  }
  println!("{:?}", true_states);
  let mut state = BitVec::new(&true_states);
  for b in initial_state.iter() {
    state.push(*b);
  }
  state.fix_prepend();
  state.fix_postpend();
  println!("{}", state);
  println!("{:?}", state);
  for i in 0..1 {
    if i % 1000 == 0 {
      println!("On generation {}", i);
      println!("Total of plant indexes: {}", state.index_totals());
    }
    state.generation();
//    println!("{}", state);
//    println!("{:?}", state);
  }
  println!("Total of plant indexes: {}", state.index_totals()); // 2542
  //2550000000883
  //322099999310706
}

#[derive(Debug, Clone)]
struct BitVec {
  bits: Vec<u64>,
  len: usize,
  bit_size: usize,
  byte_size: usize,
  vec_idx_mask: usize,
  bit_idx_count: usize,
  bit_idx_mask: usize,
  prepends: isize,
  true_states: Vec<u64>,
}

impl BitVec {
  fn new(true_states: &[u64]) -> Self {
    let byte_size = ::std::mem::size_of::<u64>();
    let bit_size = byte_size * 8;
    let mut vec_idx_mask: usize = usize::max_value();
    let mut bit_idx_count: usize = 0;
    println!("bit_size: {}", bit_size);
    for i in 0..bit_size {
      if bit_size & 1 << i > 0 {
        vec_idx_mask = vec_idx_mask << i;
        bit_idx_count = i;
      }
    }
    let bit_idx_mask = vec_idx_mask ^ usize::max_value();
    let mut true_states = Vec::from(true_states);
    true_states.sort();
    BitVec {
      bits: vec![0, 0],
      len: 0,
      bit_size,
      byte_size,
      vec_idx_mask,
      bit_idx_count,
      bit_idx_mask,
      prepends: 0,
      true_states,
    }
  }

  fn fix_prepend(&mut self) {
    if self.bits[0] & 0x1f > 0 {
      println!("Prepending");
      self.bits.insert(0, 0);
      self.prepends += self.bit_size as isize;
      self.len += self.bit_size;
    }
    while self.bits[0] + self.bits[1] == 0 {
      println!("Unprepending");
      self.bits.remove(0);
      self.prepends -= self.bit_size as isize;
      self.len -= self.bit_size;
    }
  }

  fn fix_postpend(&mut self) {
    if self.bits.last().unwrap() > &0 {
      println!("Postpending");
      self.bits.push(0);
    }
  }

  fn generation(&mut self) {
    self.fix_prepend();
    self.fix_postpend();
//    for i in 1..=self.bits.len() {
//      self.bits[i-1] = (i << (i*3)) as u64;
//    }
    let mut previous = 0u64;
    let mut current = 0u64;
    let mut next = self.bits[0];
//    let n = self.bits.len() - 1;
//    for x in 0..n {
////      println!("x={}", x);
//      previous = current;
//      current = next;
//      next = self.bits[x + 1];
//      self.bits[x] = self.compute_word(previous, current, next);
//    }
    let n = self.bits.len() - 1;
    for x in 1..n {
//      println!("x={}", x);
      previous = self.bits[x-1];
      current = *u_64;
      next = self.bits[x + 1];
      self.bits[x] = self.compute_word(previous, current, next);
    }
    self.bits[n] = self.compute_word(current, next, 0);
  }

  fn compute_word(&self, previous: u64, current: u64, next: u64) -> u64 {
    let LEFT_5: u64 = 0x1f << (self.bit_size - 5) as u64;
    if (previous & 3) + current == 0 && (next & (3 << (self.bit_size - 2))) == 0 {
//      println!("result: {:b}", 0);
      return 0;
    }
//    println!("previous: {:#066b}, current: {:#066b}, next: {:#066b}", previous, current, next);
    let mut result = 0u64;

    let mut mut_current = (previous << 62) | (current >> 2);
    let v = mut_current & LEFT_5;
//    println!("v{} = {}, {:#066b}", 63, v, mut_current);
    if let Ok(_) = self.true_states.binary_search (&v) {
      result += (1 << 63);
    }
    mut_current <<= 1;
    let v = mut_current & LEFT_5;
//    println!("v{} = {}, {:#066b}", 62, v, mut_current);
    if let Ok(_) = self.true_states.binary_search (&v) {
      result += (1 << 62);
    }
    mut_current = current;
    for i in (2..=61).rev() {
      let v = mut_current & LEFT_5;
//      println!("v{} = {}, {:#066b}", i, v, mut_current);
      if let Ok(_) = self.true_states.binary_search(&v) {
        result += (1 << i);
      }
      mut_current <<= 1;
    }
    mut_current = mut_current | (next >> 2);
    let v = mut_current & LEFT_5;
//    println!("v{} = {}, {:#066b}", 1, v, mut_current);
    if let Ok(_) = self.true_states.binary_search (&v) {
      result += (1 << 1);
    }
    mut_current <<= 1;
    let v = mut_current & LEFT_5;
//    println!("v{} = {}, {:#066b}", 0, v, mut_current);
    if let Ok(_) = self.true_states.binary_search (&v) {
      result += (1 << 0);
    }
//    println!("result: {:#066b}", result);
    result
  }

  fn index_totals(&self) -> i64 {
    let mut total = 0;
    let mut idx = 0;
    const LEFT_1: u64 = 1 << 63;
    for x in self.bits.iter() {
      let mut temp = *x;
      for j in 0 .. 64 {
        if temp & LEFT_1 > 0 {
//          println!("total={}, idx={}", total, idx);
          total += idx as i64 - self.prepends as i64;
        }
        temp = temp << 1;
        idx += 1;
      }
    }
    println!("idx = {}, total = {}", idx, total);
    total
  }

  fn push(&mut self, value: bool) -> usize {
//    println!("Pushing {} with len={}", value, self.len);
    let idx = self.len >> self.bit_idx_count;
    if self.bits.len() == idx {
//      println!("Extending bits");
      self.bits.push(0);
    }
    let bit = self.bit_idx_mask - (self.len & self.bit_idx_mask);
//    println!("bit = {}, current = {}", bit, self.bits[idx]);
    if value {
      self.bits[idx] = self.bits[idx] | 1 << bit;
//      println!("after modification = {}", self.bits[idx]);
    }
    self.len += 1;
    self.len
  }
}

impl Display for BitVec {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    for i in self.bits.iter() {
      write!(f, "{:b}_", i)?;
    }
    Ok(())
  }
}
