use regex::Regex;
use std::fmt::{Display, Error, Formatter};
use std::collections::HashSet;

pub fn compute(input: &[String]) {
  let mut map = Map::from(input);
  println!("{}", map);
  map.fill_water();
}

#[derive(Debug, Clone)]
enum MapKind {
  Sand,
  Clay,
  FlowingWater,
  StagnantWater,
}

impl Display for MapKind {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let c = match self {
      MapKind::Sand => ".",
      MapKind::Clay => "#",
      MapKind::FlowingWater => "|",
      MapKind::StagnantWater => "~",
    };
    write!(f, "{}", c)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Point {
  x: usize,
  y: usize,
}

impl Point {
  fn new(x: usize, y: usize) -> Self {
    Point { x, y }
  }
  fn down(&self) -> Self {
    Point::new(self.x, self.y + 1)
  }
  fn left(&self) -> Self {
    Point::new(self.x - 1, self.y)
  }
  fn right(&self) -> Self {
    Point::new(self.x + 1, self.y)
  }
  fn up(&self) -> Self {
    Point::new(self.x, self.y - 1)
  }
}

struct Map {
  //y then x
  map: Vec<Vec<MapKind>>,
  xmin: usize,
  ymin: usize,
  xmax: usize,
  ymax: usize,
}

impl Map {
  fn is_clay(&self, p: &Point) -> bool {
    match self.kind_at(p) {
      MapKind::Clay => true,
      _ => false,
    }
  }
  fn is_sand(&self, p: &Point) -> bool {
    match self.kind_at(p) {
      MapKind::Sand => true,
      _ => false,
    }
  }
  fn is_flowing_water(&self, p: &Point) -> bool {
    match self.kind_at(p) {
      MapKind::FlowingWater => true,
      _ => false,
    }
  }
  fn fill_water(&mut self) {
    let mut to_explore = vec![];
    let mut explored = HashSet::new();
    let mut p = Point::new(500 - self.xmin, 0);
    self.set_kind(&p, MapKind::FlowingWater);
    to_explore.push(p);
    while !to_explore.is_empty() {
      let mut p = to_explore.remove(0);
      explored.insert(p);
//      println!("{}", self);
      println!("Popped {:?}, {}", p, to_explore.len());
      println!("{:?}", to_explore);
//      ::std::thread::sleep(::std::time::Duration::from_millis(100));
      loop {
        println!("loop start: p = {:?}", p);
        if p.y >= self.map.len() - 1 {
          break;
        }
        if self.is_flowing_water(&p.down()) {
          break;
        }
        else if self.can_drop(&p) {
          p = p.down();
          if p.y >= self.map.len() - 1 {
            break;
          }
          self.set_kind(&p, MapKind::FlowingWater);
        } else {
          self.set_kind(&p, MapKind::FlowingWater);
          match self.is_contained(&p) {
            (true, true) => {
              self.fill_contained(&p);
              p = p.up();
            }
            (false, true) => {
              let mut right = p.right();
              loop {
                if self.is_clay(&right) {
                  break;
                }
                self.set_kind(&right, MapKind::FlowingWater);
                right = right.right();
              }
              loop {
                p = p.left();
                self.set_kind(&p, MapKind::FlowingWater);
                if self.can_drop(&p) {
                  break;
                }
              }
            }
            (true, false) => {
              let mut left = p.left();
              loop {
                if self.is_clay(&left) {
                  break;
                }
                self.set_kind(&left, MapKind::FlowingWater);
                left = left.left();
              }
              loop {
                p = p.right();
                self.set_kind(&p, MapKind::FlowingWater);
                if self.can_drop(&p) {
                  break;
                }
              }
            }
            // Both sides are even, so two waterfalls.
            (false, false) => {
              let mut right = p;
              loop {
                right = right.right();
                if self.is_flowing_water(&right) {
                  break;
                }
                self.set_kind(&right, MapKind::FlowingWater);
                if self.can_drop(&right) {
                  if !explored.contains(&right) {
                    println!("Adding {:?} to explore!", &right);
                    to_explore.push(right);
                  }
                  break;
                }
              }
              loop {
                p = p.left();
                if self.is_flowing_water(&p) {
                  break;
                }
                self.set_kind(&p, MapKind::FlowingWater);
                if self.can_drop(&p) {
                  break;
                }
              }
            }
          }
        }
      }
    }
    println!("{}", self);
    // remove the last line
    self.map.pop();
    let total_water: usize = self.map.iter().map(|v|v.iter().map(|mk|match mk {MapKind::FlowingWater | MapKind::StagnantWater => 1, _ => 0}).sum::<usize>()).sum();
    println!("total water: {}", total_water);
    let stagnant_water: usize = self.map.iter().map(|v|v.iter().map(|mk|match mk {MapKind::StagnantWater => 1, _ => 0}).sum::<usize>()).sum();
    println!("total water: {}", stagnant_water);
  }

  fn can_drop(&self, p: &Point) -> bool {
    self.is_sand(&p.down()) || self.is_flowing_water(&p.down())
  }

  fn is_contained(&self, p: &Point) -> (bool, bool) {
    if !self.can_drop(p) {
      let mut left = p.left();
      let mut contained_left;
      loop {
        if self.can_drop(&left) {
          contained_left = false;
          break;
        } else if self.is_clay(&left) {
          contained_left = true;
          break;
        }
        left = left.left();
      }
      let mut right = p.right();
      let mut contained_right;
      loop {
        if self.can_drop(&right) {
          contained_right = false;
          break;
        } else if self.is_clay(&right) {
          contained_right = true;
          break;
        }
        right = right.right();
      }
      return (contained_left, contained_right);
    }
    (false, false)
  }

  fn fill_contained(&mut self, p: &Point) {
    let mut right = *p;
    loop {
      if self.is_clay(&right) {
        break;
      }
      self.set_kind(&right, MapKind::StagnantWater);
      right = right.right();
    }
    let mut left = *p;
    loop {
      if self.is_clay(&left) {
        break;
      }
      self.set_kind(&left, MapKind::StagnantWater);
      left = left.left();
    }
  }

  fn kind_at(&self, p: &Point) -> &MapKind {
    &self.map[p.y][p.x]
  }

  fn set_kind(&mut self, p: &Point, k: MapKind) {
    self.map[p.y][p.x] = k;
  }
}

impl Display for Map {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    for v in self.map.iter() {
      writeln!(f)?;
      for mk in v.iter() {
        write!(f, "{}", mk)?;
      }
    }
    Ok(())
  }
}

impl From<&[String]> for Map {
  fn from(input: &[String]) -> Self {
    let regex = Regex::new(r"(x|y)=(\d+), (x|y)=(\d+)\.\.(\d+)").unwrap();
    let mut x;
    let mut y;
    let mut xmin = usize::max_value();
    let mut ymin = usize::max_value();
    let mut xmax = usize::min_value();
    let mut ymax = usize::min_value();
    let mut ranges = vec![];
    for line in input.iter() {
      let c = regex.captures(line).unwrap();
      let u = |i| -> usize { c.get(i).unwrap().as_str().parse().unwrap() };
      let _2 = u(2);
      let _4 = u(4);
      let _5 = u(5);
      if c.get(1).unwrap().as_str() == "x" {
        x = _2..=_2;
        y = _4..=_5;
      } else {
        y = _2..=_2;
        x = _4..=_5;
      }
      if x.start() < &xmin {
        xmin = *x.start();
      }
      if y.start() < &ymin {
        ymin = *y.start();
      }
      if x.end() > &xmax {
        xmax = *x.end();
      }
      if y.end() > &ymax {
        ymax = *y.end();
      }
      //      println!("{:?}, {:?}", x, y);
      ranges.push((x, y));
    }
    //    println!("{},{}  {},{}", xmin, ymin, xmax, ymax);
    // Add 10 x's to either side just in case
    xmax += 10;
    xmin -= 10;
    let mut map = vec![vec![MapKind::Sand; xmax - xmin + 1]; ymax - ymin + 2];
    for (xs, ys) in ranges.iter() {
      //For some reason can't iterate over a RangeInclusive
      for y in *ys.start()..=*ys.end() {
        for x in *xs.start()..=*xs.end() {
          map[y - ymin][x - xmin] = MapKind::Clay;
        }
      }
    }
    Map {
      map,
      xmin,
      ymin,
      xmax,
      ymax,
    }
  }
}
