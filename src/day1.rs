use ::std::fs::{File};
use ::std::io::{BufRead, BufReader};
use std::path::PathBuf;
pub fn compute() -> Result<(), ::std::io::Error> {
  let v = super::util::file_lines("inputs/day1.txt");
  let changes: Vec<i64> = v.iter().map(|s|s.parse().unwrap()).collect();
  let mut set = ::std::collections::HashSet::<i64>::new();
  let mut total = 0;
  for i in &changes {
    total += i;
  }
  println!("Total frequency after 1 run: {}", total);
  compute_duplicate(&changes);
  return Ok(());
}

fn compute_duplicate(v: &Vec<i64>) -> Result<(), ::std::io::Error> {
  let mut set = ::std::collections::HashSet::<i64>::new();
  let mut total = 0;
  loop {
    for i in v {
      total += i;
      if set.contains(&total) {
        println!("First duplicate: {}", &total);
        return Ok(());
      } else {
        set.insert(total);
      }
    }
  }
}