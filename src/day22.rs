pub fn compute() {
  let depth = 3558;
  let target = (15, 740);

  let mut cave = Cave::new(depth, target.0, target.1);

  println!("Risk level: {}", cave.risk_level());
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct GeologicIndex(u32);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct ErosionLevel(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    (x, y) => GeologicIndex(x_1.0 as u32 * y_1.0 as u32),
  }
}

fn erosion_level(geo_index: GeologicIndex, depth: usize) -> ErosionLevel {
  ErosionLevel(((geo_index.0 + depth as u32) % 20183) as u16)
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
    let x_count = x * 2 + 1;
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
