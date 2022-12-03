use std::cmp::Ordering;

pub fn compute() {
  let depth = 3558;
  let target = (15, 740);

  let mut cave = Cave::new(depth, target.0, target.1);

  println!("Risk level: {}", cave.risk_level());

  let mut searcher = Searcher::new(cave);

  println!("Minutes to target: {} (1027 is too high)", searcher.search());
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct GeologicIndex(u32);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct ErosionLevel(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Region {
  Rocky,
  Narrow,
  Wet,
  Uninit,
}

impl Region {
  fn from_erosion_level(e: ErosionLevel) -> Self {
    match e.0 % 3 {
      0 => Self::Rocky,
      1 => Self::Wet,
      2 => Self::Narrow,
      _ => unreachable!(),
    }
  }
  fn risk(self) -> u32 {
    match self {
      Self::Rocky => 0,
      Self::Narrow => 2,
      Self::Wet => 1,
      _ => unreachable!(),
    }
  }
}

fn geologic_index(x: usize, y: usize, target_x: usize, target_y: usize, x_1: ErosionLevel, y_1: ErosionLevel) -> GeologicIndex {
  if x == target_x && y == target_y {
    return GeologicIndex(0);
  }
  match (x, y) {
    (0, 0) => GeologicIndex(0),
    (x, 0) => GeologicIndex(x as u32 * 16807),
    (0, y) => GeologicIndex(y as u32 * 48271),
    (_, _) => GeologicIndex(x_1.0 as u32 * y_1.0 as u32),
  }
}

fn erosion_level(geo_index: GeologicIndex, depth: usize) -> ErosionLevel {
  ErosionLevel(((geo_index.0 + depth as u32) % 20183) as u16)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Tool {
  Torch,
  ClimbingGear,
  Neither,
}

impl Tool {
  fn valid_in_region(self, region: Region) -> bool {
    match region {
      Region::Rocky => self != Self::Neither,
      Region::Wet => self != Self::Torch,
      Region::Narrow => self != Self::ClimbingGear,
      _ => false,
    }
  }
  fn switch(self, current: Region, new: Region) -> Self {
    use Region::*;
    match (current, new) {
      (Rocky, Rocky) => match self {
        Self::Torch => Self::ClimbingGear,
        Self::ClimbingGear => Self::Torch,
        _ => unreachable!(),
      },
      (Wet, Wet) => match self {
        Self::ClimbingGear => Self::Neither,
        Self::Neither => Self::ClimbingGear,
        _ => unreachable!(),
      },
      (Narrow, Narrow) => match self {
        Self::Torch => Self::Neither,
        Self::Neither => Self::Torch,
        _ => unreachable!(),
      },
      (Rocky, Wet) | (Wet, Rocky) => Self::ClimbingGear,
      (Rocky, Narrow) | (Narrow, Rocky) => Self::Torch,
      (Wet, Narrow) | (Narrow, Wet) => Self::Neither,
      _ => unreachable!(),
    }
  }
}

struct Cave {
  geologic_indices: Vec<Vec<GeologicIndex>>,
  erosion_levels: Vec<Vec<ErosionLevel>>,
  regions: Vec<Vec<Region>>,
  depth: usize,
  target_x: usize,
  target_y: usize,
}

impl Cave {
  fn new(depth: usize, x: usize, y: usize) -> Self {
    let x_count = x * 5 + 1;
    let y_count = y * 2 + 1;
    let mut s = Self {
      geologic_indices: (0..x_count).map(|_| vec![GeologicIndex(0); y_count]).collect(),
      erosion_levels: (0..x_count).map(|_| vec![ErosionLevel(0); y_count]).collect(),
      regions: (0..x_count).map(|_| vec![Region::Uninit; y_count]).collect(),
      depth,
      target_x: x,
      target_y: y,
    };
    s.compute_geologic_indices_and_erosion_levels();
    s
  }
  fn compute_geologic_indices_and_erosion_levels(&mut self) {
    for x in 0..self.geologic_indices.len() {
      for y in 0..self.geologic_indices[x].len() {
        let x_1 = x.checked_sub(1).unwrap_or(0);
        let y_1 = y.checked_sub(1).unwrap_or(0);
        let geo_index = geologic_index(
          x,
          y,
          self.target_x,
          self.target_y,
          self.erosion_levels[x_1][y],
          self.erosion_levels[x][y_1],
        );
        self.geologic_indices[x][y] = geo_index;
        let el = erosion_level(geo_index, self.depth);
        self.erosion_levels[x][y] = el;
        let region = Region::from_erosion_level(el);
        self.regions[x][y] = region;
      }
    }
  }
  fn risk_level(&self) -> u32 {
    (0..=self.target_x)
      .map(|x| (0..=self.target_y).map(|y| self.regions[x][y].risk()).sum::<u32>())
      .sum()
  }
}

struct Searcher {
  cave: Cave,
  searchlings: Vec<Vec<Option<Searchling>>>,
  active: Vec<Vec<bool>>,
}

impl Searcher {
  fn new(cave: Cave) -> Self {
    let x = cave.regions.len();
    let y = cave.regions[0].len();
    let mut s = Self {
      cave,
      searchlings: (0..x).map(|_| vec![None; y]).collect(),
      active: (0..x).map(|_| vec![false; y]).collect(),
    };
    s.searchlings[0][0] = Some(Searchling {
      minutes: 0,
      tool: Tool::Torch,
    });
    s.active[0][0] = true;
    s
  }
  fn search(&mut self) -> u16 {
    for i in 0..2000 {
      println!("Loop {}", i);
      // check end state
      //if let Some(mut s) = self.searchlings[self.cave.target_x][self.cave.target_y] {
      //  if s.tool != Tool::Torch {
      //    s.minutes += 7;
      //  }
      //  return s.minutes;
      //}

      let active = self.active.clone();
      self.active.iter_mut().for_each(|v| v.iter_mut().for_each(|b| *b = false));
      active.iter().enumerate().for_each(|(x, v)| {
        v.iter().enumerate().for_each(|(y, b)| {
          if *b {
            let near_searchlings = self.near_searchlings(x, y);
            let moves = self.searchlings[x][y].unwrap().searchlings(near_searchlings);
            self.apply_searchlings(x, y, moves);
          }
        })
      })
    }
    if let Some(mut s) = self.searchlings[self.cave.target_x][self.cave.target_y] {
      if s.tool != Tool::Torch {
        s.minutes += 7;
      }
      return s.minutes;
    } else {
      0
    }
  }
  fn near_coordinates(&self, x: usize, y: usize) -> [Option<(usize, usize)>; 4] {
    let up = Some((x, y + 1)).filter(|c| c.1 < self.searchlings[0].len());
    let down = Some((x, y.saturating_sub(1))).filter(|c| c.1 < y);
    let left = Some((x.saturating_sub(1), y)).filter(|c| c.0 < x);
    let right = Some((x + 1, y)).filter(|c| c.0 < self.searchlings.len());
    [up, left, right, down]
  }
  fn near_searchlings(&mut self, x: usize, y: usize) -> [(Region, Region, Option<Searchling>); 4] {
    let current = self.cave.regions[x][y];
    let [n1, n2, n3, n4] = self.near_coordinates(x, y);
    let c = |c: Option<(usize, usize)>| {
      c.map_or((current, Region::Uninit, None), |(x, y)| {
        (current, self.cave.regions[x][y], self.searchlings[x][y])
      })
    };
    [c(n1), c(n2), c(n3), c(n4)]
  }
  fn apply_searchlings(&mut self, x: usize, y: usize, near: [Option<Searchling>; 4]) {
    near
      .iter()
      .copied()
      .zip(self.near_coordinates(x, y).iter().copied())
      .for_each(|(s, c)| {
        if let Some(s) = s {
          let (x, y) = c.unwrap();
          self.active[x][y] = true;
          self.searchlings[x][y] = Some(s);
          if x == self.cave.target_x && y == self.cave.target_y {
            println!("Set searchling at {}, {} to {:?}", x, y, s);
          }
        }
      })
  }
}

#[derive(Clone, Copy, Debug)]
struct Searchling {
  minutes: u16,
  tool: Tool,
}

impl Searchling {
  fn searchlings(self, moves: [(Region, Region, Option<Searchling>); 4]) -> [Option<Searchling>; 4] {
    let [m1, m2, m3, m4] = moves;
    [
      self.search(m1.0, m1.1, m1.2),
      self.search(m2.0, m2.1, m2.2),
      self.search(m3.0, m3.1, m3.2),
      self.search(m4.0, m4.1, m4.2),
    ]
  }
  fn search(self, current: Region, next: Region, searchling: Option<Searchling>) -> Option<Searchling> {
    if next == Region::Uninit {
      return None;
    }
    let new = if self.tool.valid_in_region(next) {
      Searchling {
        minutes: self.minutes + 1,
        tool: self.tool,
      }
    } else {
      Searchling {
        minutes: self.minutes + 8,
        tool: self.tool.switch(current, next),
      }
    };
    //println!("Searchling {:?}, replacing: {:?}", new, searchling);
    //searchling.filter(|s| new <= *s)
    Some(new).filter(|n| searchling.map_or(true, |s| *n < s))
  }
}

impl Ord for Searchling {
  fn cmp(&self, other: &Self) -> Ordering {
    self.minutes.cmp(&other.minutes)
  }
}

impl PartialOrd for Searchling {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.minutes.partial_cmp(&other.minutes)
  }
}

impl PartialEq for Searchling {
  fn eq(&self, other: &Self) -> bool {
    self.minutes.eq(&other.minutes)
  }
}

impl Eq for Searchling {}

#[test]
fn test_geologic_indices_and_erosion_levels() {
  let mut cave = Cave::new(510, 10, 10);
  assert_eq!(cave.geologic_indices[0][0].0, 0);
  assert_eq!(cave.geologic_indices[1][0].0, 16807);
  assert_eq!(cave.geologic_indices[0][1].0, 48271);
  assert_eq!(cave.geologic_indices[1][1].0, 145722555);
  assert_eq!(cave.geologic_indices[10][10].0, 0);

  assert_eq!(cave.erosion_levels[0][0].0, 510);
  assert_eq!(cave.erosion_levels[1][0].0, 17317);
  assert_eq!(cave.erosion_levels[0][1].0, 8415);
  assert_eq!(cave.erosion_levels[1][1].0, 1805);
  assert_eq!(cave.erosion_levels[10][10].0, 510);

  assert_eq!(cave.regions[0][0], Region::Rocky);
  assert_eq!(cave.regions[1][0], Region::Wet);
  assert_eq!(cave.regions[0][1], Region::Rocky);
  assert_eq!(cave.regions[1][1], Region::Narrow);
  assert_eq!(cave.regions[10][10], Region::Rocky);
}
