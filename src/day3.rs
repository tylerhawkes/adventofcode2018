use nom::{digit, space0, types::CompleteStr};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Claim {
  id: usize,
  //  left: usize,
  //  top: usize,
  width: usize,
  height: usize,
  xmin: usize,
  ymin: usize,
  xmax: usize,
  ymax: usize,
}

impl Claim {
  fn new(id: usize, left: usize, top: usize, width: usize, height: usize) -> Self {
    Claim {
      id,
      width,
      height,
      xmin: left,
      ymin: top,
      xmax: left + width - 1,
      ymax: top + height - 1,
    }
  }
  //  fn overlaps(&self, other: &Claim) -> bool {
  //    self.overlaps_point(other.xmin, other.ymin) ||
  //      self.overlaps_point(other.xmin, other.ymax) ||
  //      self.overlaps_point(other.xmax, other.ymax) ||
  //      self.overlaps_point(other.xmax, other.ymin) ||
  //      self.xmin >= other.xmin && self.xmax <= other.xmax && self.ymin >= other.ymin && self.ymax <= other.ymax ||
  //      self.xmin <= other.xmin && self.xmax >= other.xmax && self.ymin <= other.ymin && self.ymax >= other.ymax
  //  }
  //  fn overlaps_point(&self, x: usize, y: usize) -> bool {
  //    self.xmin <= x && self.xmax >= x && self.ymin <= y && self.ymax >= y
  //  }
}

struct Fabric {
//  width: usize,
//  height: usize,
  // Outer vec is columns, inner is rows, like x and y
  square_inches: Vec<Vec<u8>>,
}

impl Fabric {
  fn new(width: usize, height: usize) -> Self {
    let mut x = vec![];
    for _i in 0..width {
      let y = vec![0u8; height];
      x.push(y);
    }
    Fabric {
      square_inches: x,
    }
  }

  fn mark_claim(&mut self, claim: &Claim) {
    for x in claim.xmin..=claim.xmax {
      for y in claim.ymin..=claim.ymax {
        self.square_inches[x][y] += 1;
      }
    }
  }

  fn claim_overlaps(&mut self, claim: &Claim) -> bool {
    let mut count: usize = 0;
    for x in claim.xmin..=claim.xmax {
      for y in claim.ymin..=claim.ymax {
        count += self.square_inches[x][y] as usize;
      }
    }
    count == claim.width * claim.height
  }
}

pub fn compute(input: &[String]) {
  let claims: Vec<Claim> = input.iter().map(|s| parse(s)).collect();
  let mut fabric = Fabric::new(1000, 1000);
  println!("Claims length: {}", claims.len());
  for claim in &claims {
    fabric.mark_claim(claim);
  }
  //let total_overlapping: u64 = fabric.square_inches.iter().map(|x| x.iter().map(|y|if y > &1 {1u64} else {0u64}).sum()).sum();
  let mut total_overlapping = 0u64;
  for x in &fabric.square_inches {
    for y in x {
      if y > &1 {
        total_overlapping += 1;
      }
    }
  }
  println!("Total overlapping = {}", total_overlapping);

  //  Still would like to figure out why this is reporting incorrect results.
  //  for left in &claims {
  //    let mut overlaps = false;
  //    for right in &claims {
  //      if left.id != right.id && (left.overlaps(right) || right.overlaps(left)) {
  //        overlaps = true;
  //        //println!("Claim {} overlaps with claim {}", left.id, right.id);
  //        break;
  //      }
  //    }
  //    if !overlaps {
  //      println!("Claim id not overlapping: {}", left.id);
  //    }
  //  }

  for claim in &claims {
    if fabric.claim_overlaps(claim) {
      println!("Claim not overlapping => {}", claim.id);
    }
  }

  //  let xstart = 750;
  //  let ystart = 900;
  //  for y in ystart..fabric.height.min(ystart+60) {
  //    for x in xstart..fabric.width.min(xstart+250) {
  //      let a = &fabric.square_inches[x][y];
  //      let b = if a == &0 {" ".to_owned()} else {format!("{}", a)};
  //      print!("{}", b);
  //    }
  //    println!()
  //  }
}

fn parse(s: &str) -> Claim {
  match claim_parser(CompleteStr(s)) {
    Ok((_i, o)) => o,
    Err(e) => panic!(format!("Unable to parse {}, got {}", s, e)),
  }
}

#[test]
fn test_parse() {
  assert_eq!(parse("#1 @ 12,548: 19x10"), Claim::new(1, 12, 548, 19, 10));
}

fn to_int(c: CompleteStr) -> Result<usize, ::std::num::ParseIntError> {
  usize::from_str(c.as_ref())
}

named!(integer(CompleteStr) -> usize,
  map_res!(digit, to_int)
);

named! {claim_parser<CompleteStr, Claim>,
  do_parse!(
    tag_s!("#") >>
    id: integer >>
    space0 >>
    tag_s!("@") >>
    space0 >>
    left: integer >>
    tag_s!(",") >>
    top: integer >>
    tag_s!(":") >>
    space0 >>
    width: integer >>
    tag_s!("x") >>
    height: integer >>
    (Claim::new(id, left, top, width, height))
  )
}
