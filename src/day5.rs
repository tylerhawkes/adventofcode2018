pub fn compute(input: &[String]) {
  let mut s = input.first().unwrap().clone();
  //println!("{}", &s);
  while let Some(()) = react(&mut s) {}
  println!("{}, {}", s.len(), s);
  let mut char_set = ::std::collections::HashSet::<char>::new();
  for c in (&s).chars() {
    char_set.insert(c.to_ascii_lowercase());
  }

  let mut min_size: usize = 0xffffffffffffffff;
  let mut min_char = '-';
  for c in char_set.iter() {
    let size = reduce(*c, &s);
    if size < min_size {
      min_size = size;
      min_char = *c;
    }
  }
  println!("Min length is {} for char {}", min_size, min_char);
}

fn react(s: &mut String) -> Option<()> {
  let mut p = '-';
  for (i, c) in s.chars().enumerate() {
    if c.to_ascii_uppercase() == p.to_ascii_uppercase() && c != p {
      s.remove(i - 1);
      s.remove(i - 1);
      //println!("{}", s);
      return Some(());
    }
    p = c;
  }
  None
}

fn reduce(lower_char: char, s: &String) -> usize {
  let mut s = s.clone();
  s = s.replace(lower_char, "");
  s = s.replace(lower_char.to_ascii_uppercase(), "");
  while let Some(()) = react(&mut s) {}
  s.len()
}
