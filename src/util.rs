use std::fs::File;
use std::io::{BufRead, BufReader, Read};
pub fn file_lines(file: &str) -> Vec<String> {
  let input = File::open(file).unwrap_or_else(|_| panic!(format!("Unable to open file {}", file)));
  let reader = BufReader::new(&input);
  let mut v = vec![];
  for line in reader.lines() {
    v.push(line.expect("Unable to read string!"));
  }
  v
}

pub fn file_string(file: &str) -> String {
  let mut ret = String::new();
  let mut input = File::open(file).unwrap_or_else(|_| panic!(format!("Unable to open file {}", file)));
  input
    .read_to_string(&mut ret)
    .unwrap_or_else(|_| panic!("Unable to read to string!"));
  ret
}
