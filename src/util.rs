use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::fs::File;
pub fn file_lines(file: &str) -> Vec<String> {
  let input = File::open(file).expect(&format!("Unable to open file {}", file));
  let reader = BufReader::new(&input);
  let mut v = vec![];
  for line in reader.lines() {
    v.push(line.expect("Unable to read string!"));
  }
  return v;
}