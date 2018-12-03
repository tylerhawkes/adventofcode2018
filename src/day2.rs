pub fn compute() {
  let ids = super::util::file_lines("inputs/day2.txt");
  let mut double_count = 0;
  let mut triple_count = 0;
  for id in &ids {
    let (double, triple) = double_triple_count(id);
    if double {
      double_count += 1;
    }
    if triple {
      triple_count += 1;
    }
  }
  println!("Checksum is {}", double_count * triple_count);
}

fn double_triple_count(s: &str) -> (bool, bool) {
  let mut counts = ::std::collections::HashMap::<char, i64>::new();
  for c in s.chars() {
    let mut update = 0;
    {
      let res = counts.get(&c);
      match res {
        Some(i) => {
          let update = i + 1;
        }
        None => {
          update = 1;
        }
      }
    }
    counts.insert(c, update);
  }
  let double = counts.iter().any(|e| { e.1 == &2 });
  let triple = counts.iter().any(|e| { e.1 == &3 });
  (double, triple)
}