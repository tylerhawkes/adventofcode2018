pub fn compute(input: &[String]) {
  let changes: Vec<i64> = input.iter().map(|s| s.parse().unwrap()).collect();
  let _set = ::std::collections::HashSet::<i64>::new();
  let mut total = 0;
  for i in &changes {
    total += i;
  }
  println!("Total frequency after 1 run: {}", total);
  compute_duplicate(&changes);
}

fn compute_duplicate(v: &[i64]) {
  let mut set = ::std::collections::HashSet::<i64>::new();
  let mut total = 0;
  for _j in 1..1000 {
    for i in v {
      total += i;
      if set.contains(&total) {
        println!("First duplicate: {}", &total);
        return;
      } else {
        set.insert(total);
      }
    }
  }
}
