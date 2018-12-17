use std::collections::HashMap;

type States = HashMap<(bool, bool, bool, bool, bool), bool>;

pub fn compute(input: &[String]) {
  let mut iter = input.iter();
  let initial_state = (&iter.next().unwrap().clone()[15..]).to_owned();
  let initial_state: Vec<bool> = initial_state.chars().map(|c| c == '#').collect();
  iter.next();
  let mut states = HashMap::with_capacity(iter.size_hint().0);
  for rule in iter {
    let k = &rule[0..5];
    let k: Vec<bool> = k.chars().map(|c| c == '#').collect();
    let k = (k[0], k[1], k[2], k[3], k[4]);
    let v = &rule[rule.len() - 1..];
    let v = v == "#";
    states.insert(k, v);
  }
  println!("{:?}", initial_state);
  println!("{:?}", states);
  states.iter().for_each(|(k, v)| println!("{:?} => {}", k, v));

  let mut s = initial_state.clone();
  let mut prepends = maybe_prepend(&mut s);
  maybe_postpend(&mut s);
  for i in 0..20 {
    if i %10000 = 0 {
      print_state(&s, i);
    }
    prepends += compute_generation(&states, &mut s);
  }
  print_state(&s, 20);
  let mut sum = 0;
  for (i, b) in s.iter().enumerate() {
    if *b {
      sum += i as isize - prepends;
    }
  }
  println!("Total of plant indexes: {}", sum); // 2542
}

fn print_state(s: &[bool], iter: usize) {
  let mut t = String::new();
  s.iter().for_each(|b| if *b { t.push('#') } else { t.push('.') });
  println!("{}: {}", iter, t);
}

fn compute_generation(states: &States, s: &mut Vec<bool>) -> isize {
  let mut temp = vec![false; s.len()];
  let prepend = maybe_prepend(s);
  maybe_postpend(s);
  for i in 0..s.len() - 5 {
    let k = (s[i], s[i + 1], s[i + 2], s[i + 3], s[i + 4]);
    let v = *states.get(&k).unwrap();
    //println!("Setting {} to {} because of {:?}", s[i+2], v, k);
    temp[i + 2] = v;
  }
  for (i, b) in temp.iter().enumerate() {
    s[i] = *b;
  }
  prepend
}

fn maybe_prepend(s: &mut Vec<bool>) -> isize {
  if s[0] || s[1] || s[2] || s[3] || s[4] {
    s.insert(0, false);
    return maybe_prepend(s) + 1;
  }
  0
}

fn maybe_postpend(s: &mut Vec<bool>) {
  let len = s.len();
  if s[len-5] || s[len-4] || s[len - 3] || s[len-2] || s[len-1]{
    s.push(false);
    maybe_postpend(s);
  }
}
