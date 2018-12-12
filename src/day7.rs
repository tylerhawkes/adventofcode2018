use std::collections::{HashSet};

#[derive(Debug, Copy, Clone)]
struct Dependency {
  parent: char,
  child: char,
}

pub fn compute(input: &[String]) {
  let dependencies: Vec<Dependency> = input.iter().map(|s|{
    let mut chars = s.chars();
    let parent = chars.nth(5).unwrap();
    let child = chars.nth(30).unwrap();
    Dependency{parent, child}
  }).collect();
  //dependencies.iter().for_each(|d|println!("{} -> {}", d.parent, d.child));
  compute_part1(&dependencies);
  compute_part2(&dependencies);
}

//TODO: Solve this with a graph
fn compute_part1(dependencies: &[Dependency]) {
  let mut dependencies: Vec<Dependency> = Vec::from(dependencies);
  let mut cont = true;
  let mut set = HashSet::with_capacity(26);
  for l in (b'A' ..= b'Z').map(char::from) {
    set.insert(l);
  }
  let mut order = String::with_capacity(set.len());
  while cont {
    let (p, c) = sets_from_dependencies(&dependencies);
    let mut available: Vec<&char> = p.difference(&c).collect();
    available.sort();
//    println!("available {:?}", available);
    if let Some(l) = available.first() {
      dependencies = dependencies.into_iter().filter(|d|d.parent!=**l).collect();
      order.push(**l);
      set.remove(l);
    }
    cont = dependencies.len() > 0;
  }
  let mut available: Vec<&char> = set.iter().collect();
  available.sort();
  available.into_iter().for_each(|l|order.push(*l));
  println!("The order to complete tasks is {}", order);
}

fn sets_from_dependencies(d: &[Dependency]) -> (HashSet<char>, HashSet<char>) {
  let parents: HashSet<char> = d.iter().map(|d|d.parent).collect();
  let children: HashSet<char> = d.iter().map(|d|d.child).collect();
  (parents, children)
}

#[derive(Debug, Copy, Clone)]
struct Work {
  c: char,
  time_left: u8,
}

impl Work {
  fn new(c: char) -> Self {
    Work{c, time_left: 61 + (c as u32 - 'A' as u32) as u8}
  }
}

fn compute_part2(dependencies: &[Dependency]) {
  let mut dependencies: Vec<Dependency> = Vec::from(dependencies);
  let mut cont = true;
  let mut set = HashSet::with_capacity(26);
  for l in (b'A' ..= b'Z').map(char::from) {
    set.insert(l);
  }
  let mut work: Vec<Work> = Vec::with_capacity(5);
  let mut seconds = 0;
  let limit = 5;
  while cont {
    let finished: Vec<Work> = work.iter().filter(|w|w.time_left == 0).map(|w|w.clone()).collect();
    if finished.len() > 0 || work.is_empty() {
      // remove whatever is finished.
      for f in finished.iter() {
        work = work.into_iter().filter(|w|w.c != f.c).collect();
        dependencies = dependencies.into_iter().filter(|d| d.parent != f.c).collect();
        set.remove(&f.c);
      }
      if dependencies.len() == 0 {
        set.iter().for_each(|l| dependencies.push(Dependency{parent: *l, child: 'a'}))
      }
      let (p, c) = sets_from_dependencies(&dependencies);
      let mut available: Vec<&char> = p.difference(&c).collect();
      available.sort();
//      println!("available {:?}", available);
      for i in 0 .. (limit - work.len()).min(available.len()) {
        let l = available[i];
        if !work.iter().any(|w|w.c == *l) {
          work.push(Work::new(*l));
        }
      }
    }
    seconds += 1;
    work.iter_mut().for_each(|w|w.time_left = w.time_left - 1);
//    println!("work: {:?}", &work);
    cont = work.len() > 0;
  }
  println!("It takes {} seconds to complete the work with {} helpers", seconds - 1, limit);
}