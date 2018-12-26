use std::fmt::{Display, Formatter, Error};

pub fn compute(input: &[String]) {
  part1(input);
  part2(input);
}

fn part1(input: &[String]) {
  let mut map = Map::from(input);
  let acre = Acre{x: 0, y: 2};
  println!("{}", map);
  for i in 0..10 {
    map.tick();
    println!("iter = {}", i+1);
    println!("{}", map);
  }
  let trees = map.map_kind_count(MapKind::Trees);
  let lumber_yards = map.map_kind_count(MapKind::LumberYard);
  println!("trees: {}, lumber yards: {}, product: {}", trees, lumber_yards, trees * lumber_yards);
}

fn part2(input: &[String]) {
  let mut map = Map::from(input);
  let mut resource_values = Vec::with_capacity(256);
  for i in 0..1000 {
    map.tick();
    if i %1000 == 0 {
//      println!("iter = {}\n{}", i, map);
    }
    let trees = map.map_kind_count(MapKind::Trees);
    let lumber_yards = map.map_kind_count(MapKind::LumberYard);
    resource_values.push(trees * lumber_yards);
//    println!("rv: {}", trees * lumber_yards);
  }
  let mut frequency = 0;
  let last = resource_values.last().unwrap();
  for (i, rv) in resource_values.iter().rev().enumerate() {
    if i != 0 && rv == last {
      frequency = i;
      break;
    }
  }
  println!("After 1 billion minutes there are {} resource values", resource_values[resource_values.len() - ((1000000000-1000) % frequency) - 1]);
  //202860 not right
  // 201341
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MapKind {
  Trees,
  LumberYard,
  Open,
}

impl From<char> for MapKind {
  fn from(c: char) -> Self {
    match c {
      '.' => MapKind::Open,
      '|' => MapKind::Trees,
      '#' => MapKind::LumberYard,
      _ => panic!(format!("Unknown character '{}'", c))
    }
  }
}

impl Display for MapKind {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let c = match self {
      MapKind::Open => '.',
      MapKind::Trees => '|',
      MapKind::LumberYard => '#'
    };
    write!(f, "{}", c)
  }
}

#[derive(Debug, Clone)]
struct Acre {
  x: usize,
  y: usize,
}

impl Acre {
  fn neighbors(&self, xlen: usize, ylen: usize) -> Vec<Self> {
    let mut acres = Vec::with_capacity(8);
    let x = self.x as isize;
    let y = self.y as isize;
    acres.push((x-1, y-1));
    acres.push((x  , y-1));
    acres.push((x+1, y-1));
    acres.push((x-1, y  ));
    acres.push((x+1, y  ));
    acres.push((x-1, y+1));
    acres.push((x  , y+1));
    acres.push((x+1, y+1));
    acres.into_iter()
      .filter(|(x, y)| x >= &0 && x < &(xlen as isize) && y >= &0 && y < &(ylen as isize))
      .map(|(x, y)| Acre {x: x as usize, y: y as usize})
      .collect()
  }
}

struct Map {
  map: Vec<Vec<MapKind>>,
  ylen: usize,
  xlen: usize,
}

impl Map {
  fn map_kind_neighbors(&self, acre: &Acre) -> Vec<MapKind> {
    let neighbors = acre.neighbors(self.xlen, self.ylen);
    neighbors.iter().map(|a|*self.kind_at(a)).collect()
  }
  fn lumber_yard_neighbor_count(&self, acre: &Acre) -> usize {
    self.neighbor_count(acre, MapKind::LumberYard)
  }
  fn tree_neighbor_count(&self, acre: &Acre) -> usize {
    self.neighbor_count(acre, MapKind::Trees)
  }
  fn neighbor_count(&self, acre: &Acre, map_kind: MapKind) -> usize {
    self.map_kind_neighbors(acre).iter().filter(|mk|mk == &&map_kind).count()
  }
  fn kind_at(&self, acre: &Acre) -> &MapKind {
    &self.map[acre.y][acre.x]
  }
  fn map_kind_count(&self, map_kind: MapKind) -> usize {
    self.map.iter().map(|l|l.iter().filter(|mk|mk == &&map_kind).count()).sum()
  }
  fn map_after_tick(&self) -> Vec<Vec<MapKind>> {
    let mut result = Vec::with_capacity(self.map.len());
    for (y, line) in self.map.iter().enumerate() {
      result.push(Vec::with_capacity(line.len()));
      for (x, mk) in line.iter().enumerate() {
        let acre = Acre {x, y};
        let next_mk = match mk {
          MapKind::Open => {
            if self.tree_neighbor_count(&acre) >= 3 {
              MapKind::Trees
            } else {
              MapKind::Open
            }
          },
          MapKind::Trees => {
            if self.lumber_yard_neighbor_count(&acre) >= 3 {
              MapKind::LumberYard
            } else {
              MapKind::Trees
            }
          },
          MapKind::LumberYard => {
            if self.tree_neighbor_count(&acre) >= 1 && self.lumber_yard_neighbor_count(&acre) >= 1 {
              MapKind::LumberYard
            } else {
              MapKind::Open
            }
          },
        };
        result[y].push(next_mk);
      }
    }
    result
  }
  fn tick(&mut self) {
    self.map = self.map_after_tick();
  }
}

impl From<&[String]> for Map {
  fn from(input: &[String]) -> Self {
    let mut map = Vec::with_capacity(input.len());
    for (y, line) in input.iter().enumerate() {
      map.push(Vec::with_capacity(line.len()));
      for (x, c) in line.chars().enumerate() {
        map[y].push(MapKind::from(c));
      }
    }
    let xlen = map[0].len();
    let ylen = map.len();
    Map { map, xlen, ylen }
  }
}

impl Display for Map {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    for l in self.map.iter() {
      for mk in l.iter() {
        write!(f, "{}", mk)?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}