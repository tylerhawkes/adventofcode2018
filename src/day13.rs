use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
  x: usize,
  y: usize,
}

struct Map {
  tracks: Vec<Vec<Track>>,
}

impl Map {
  fn track_at(&self, p: &Point) -> Track {
    self.tracks[p.y][p.x]
  }
}

#[derive(Debug)]
struct Carts {
  carts: Vec<Cart>,
}

impl Carts {
  fn sort(&mut self) {
    self.carts.sort_by(|a, b| {
      let ordering = a.point.x.cmp(&b.point.x);
      if let Ordering::Equal = ordering {
        return a.point.y.cmp(&b.point.y);
      }
      ordering
    });
  }

  fn tick_part_one(&mut self, map: &Map) -> Option<Point> {
    self.sort();
    for i in 0..self.carts.len() {
      self.carts[i].tick(map);
      let crashed: Vec<Point> = self.carts.iter().filter(|c| c.point == self.carts[i].point).map(|c| c.point).collect();
      if crashed.len() > 1 {
        return Some(self.carts[i].point);
      }
    }
    None
  }

  fn tick_part_two(&mut self, map: &Map) -> usize {
    self.sort();
    let mut i = 0;
    while i < self.carts.len() {
      self.carts[i].tick(map);
      let removals: Vec<usize> = self.carts.iter().enumerate().filter(|(_j,c)| c.point == self.carts[i].point && c.id != self.carts[i].id).map(|(j, _c)|j).collect();
      if !removals.is_empty() {
        if removals.len() > 1 {
          panic!("More than 2 carts in a crash!");
        }
        let other = *removals.first().unwrap();
        let mut removed = vec![];
        if other < i {
          removed.push(self.carts.remove(i));
          removed.push(self.carts.remove(other));
          i -= 1;
        } else {
          removed.push(self.carts.remove(other));
          removed.push(self.carts.remove(i));
        }
//        removed.iter().for_each(|c| println!("Removed cart: {:?}", c));
      } else {
        i += 1;
      }
    }
    self.carts.len()
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Track {
  Empty,
  Vertical,
  Horizontal,
  Intersection,
  TurnRight,
  //On a circle at the origin this is quadrant 1 so right
  TurnLeft, //On a circle at the origin this is quadrant 2 so left
}

impl Track {
  fn direction(&self, direction: Direction, turn: &Turn) -> Direction {
    match self {
      Track::Empty => panic!("Cannot be on empty!"),
      Track::Vertical => {
        match direction {
          Direction::North => Direction::North,
          Direction::South => Direction::South,
          _ => panic!("Invalid vertical direction!"),
        }
      }
      Track::Horizontal => {
        match direction {
          Direction::West => Direction::West,
          Direction::East => Direction::East,
          _ => panic!("Invalid horizontal direction!"),
        }
      }
      Track::Intersection => {
        match (direction, turn) {
          (Direction::North, Turn::Left) => Direction::East,
          (Direction::North, Turn::Straight) => Direction::North,
          (Direction::North, Turn::Right) => Direction::West,
          (Direction::South, Turn::Left) => Direction::West,
          (Direction::South, Turn::Straight) => Direction::South,
          (Direction::South, Turn::Right) => Direction::East,
          (Direction::West, Turn::Left) => Direction::North,
          (Direction::West, Turn::Straight) => Direction::West,
          (Direction::West, Turn::Right) => Direction::South,
          (Direction::East, Turn::Left) => Direction::South,
          (Direction::East, Turn::Straight) => Direction::East,
          (Direction::East, Turn::Right) => Direction::North,
        }
      }
      Track::TurnRight => {
        match direction {
          Direction::North => Direction::East,
          Direction::South => Direction::West,
          Direction::West => Direction::South,
          Direction::East => Direction::North,
        }
      }
      Track::TurnLeft => {
        match direction {
          Direction::North => Direction::West,
          Direction::South => Direction::East,
          Direction::West => Direction::North,
          Direction::East => Direction::South,
        }
      }
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Turn {
  Left,
  Straight,
  Right,
}

impl Turn {
  fn next(&self) -> Self {
    match self {
      Turn::Left => Turn::Straight,
      Turn::Straight => Turn::Right,
      Turn::Right => Turn::Left,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
  North,
  South,
  East,
  West,
}

impl Direction {
  fn next_coord(&self, p: &Point) -> Point {
    match self {
      Direction::North => Point { x: p.x, y: p.y - 1 },
      Direction::South => Point { x: p.x, y: p.y + 1 },
      Direction::East => Point { x: p.x - 1, y: p.y },
      Direction::West => Point { x: p.x + 1, y: p.y },
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Cart {
  id: usize,
  point: Point,
  turn: Turn,
  direction: Direction,
}

impl Cart {
  fn new(id: usize, x: usize, y: usize, direction: Direction) -> Self {
    Cart {
      id,
      point: Point { x, y },
      turn: Turn::Left,
      direction,
    }
  }

  fn tick(&mut self, map: &Map) {
    let point = self.direction.next_coord(&self.point);
    self.point = point;
    let track = map.track_at(&self.point);
    self.direction = track.direction(self.direction, &self.turn);
    if track == Track::Intersection {
      self.turn = self.turn.next();
    }
  }
}

pub fn compute(input: &[String]) {
  let mut tracks = Vec::with_capacity(input.len());
  let mut carts = vec![];
  let mut cart_id = 0;
  for (y, line) in input.iter().enumerate() {
    tracks.push(vec![Track::Empty; line.len()]);
    for (x, c) in line.chars().enumerate() {
      let track = match c {
        ' ' => Track::Empty,
        '|' => Track::Vertical,
        '-' => Track::Horizontal,
        '\\' => Track::TurnRight,
        '/' => Track::TurnLeft,
        '+' => Track::Intersection,
        '^' => {
          cart_id += 1;
          carts.push(Cart::new(cart_id, x, y, Direction::North));
          Track::Vertical
        }
        'v'|'V' => {
          cart_id += 1;
          carts.push(Cart::new(cart_id, x, y, Direction::South));
          Track::Vertical
        }
        '<' => {
          cart_id += 1;
          carts.push(Cart::new(cart_id, x, y, Direction::East));
          Track::Horizontal
        }
        '>' => {
          cart_id += 1;
          carts.push(Cart::new(cart_id, x, y, Direction::West));
          Track::Horizontal
        }
        l => panic!(format!("Unknown character '{}'", l)),
      };
      tracks[y][x] = track;
    }
  }
  let map = Map { tracks };
  let carts_part_two = carts.clone();
  let mut carts = Carts { carts };
  for i in 0..1000 {
    if i % 100 == 0 {
//      println!("On iteration {}", i);
    }
    let crashed = carts.tick_part_one(&map);
    if let Some(p) = crashed {
      println!("Crashed at {:?}", p);
      break;
    }
//    carts.carts.iter().filter(|c| c.id == 1).for_each(|c| println!("{:?}", c));
  }
  let mut carts = Carts { carts: carts_part_two };
  let mut cart_count = usize::max_value();
  let mut iter = 0;
  while cart_count > 1 {
    if iter % 100 == 0 {
//      println!("On iter {}", iter);
    }
    cart_count = carts.tick_part_two(&map);
    iter += 1;
  }
  println!("Last cart at {:?}", carts.carts[0].point);
  println!("Carts: {:?}", carts);
}
