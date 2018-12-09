use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn file_lines(file: &str) -> Vec<String> {
  let input = File::open(file).unwrap_or_else(|_| panic!("Unable to open file {}", file));
  let reader = BufReader::new(&input);
  let mut v = vec![];
  for line in reader.lines() {
    v.push(line.expect("Unable to read string!"));
  }
  v
}
