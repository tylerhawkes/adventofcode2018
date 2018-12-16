pub fn compute(input: &[String]) {
  let ids = input;
  let mut double_count = 0;
  let mut triple_count = 0;
  for id in ids {
    let (double, triple) = double_triple_count(id);
    if double {
      double_count += 1;
    }
    if triple {
      triple_count += 1;
    }
  }
  println!(
    "Double: {}, triple: {}, checksum is {}",
    double_count,
    triple_count,
    double_count * triple_count
  );

  let cp: Vec<(&String, &String)> = ids
    .iter()
    .flat_map(|s| {
      let mut v = Vec::with_capacity(ids.len());
      let mut append = false;
      for id in ids {
        if append {
          v.push((s, id))
        }
        if s == id {
          append = true;
        }
      }
      v
    })
    .collect();
  for (s, t) in cp.iter() {
    if ::strsim::levenshtein(s, t) == 1 {
      println!("{}, {}", s, t);
      print_same(s, t);
    }
  }
  println!("Length is {}", cp.len());
}

fn double_triple_count(s: &str) -> (bool, bool) {
  let mut counts = ::std::collections::HashMap::<char, i64>::new();
  for c in s.chars() {
    let update;
    {
      let res = counts.get(&c);
      match res {
        Some(i) => {
          update = i + 1;
        }
        None => {
          update = 1;
        }
      }
    }
    counts.insert(c, update);
  }
  let double = counts.iter().any(|e| *e.1 == 2);
  let triple = counts.iter().any(|e| *e.1 == 3);
  //  for (k, v) in counts.iter() {
  //    print!("{}={},", k, v);
  //  }
  //  println!("{}, {}", triple, double);
  (double, triple)
}

fn print_same(s: &str, t: &str) {
  for (u, v) in s.chars().zip(t.chars()) {
    if u == v {
      print!("{}", u);
    }
  }
  println!();
}
