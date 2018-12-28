use std::collections::{HashMap, hash_map::Entry};

pub fn compute(input: &str) {
//  let rooms = Rooms::from("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$");
//  let rooms = Rooms::from("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$");
  let rooms = Rooms::from(input);
  println!("Max distance: {}", rooms.rooms.values().map(|r|r.distance).max().unwrap());
  println!("Rooms with min distance >= 1000: {}", rooms.rooms.values().filter(|r|r.distance >= 1000).count());
}

#[derive(Debug, Clone)]
struct Rooms {
  rooms: HashMap<Grid, Room>,
}

impl From<&str> for Rooms {
  fn from(input: &str) -> Self {
    let mut rooms = HashMap::new();
    let origin_grid = Grid{x: 0, y: 0};
    let initial_room = Room::new(origin_grid, Direction::None, 0);
    rooms.insert(initial_room.grid, initial_room);
    let mut current_grid = origin_grid;
    let mut parens = Vec::new();
    let mut grow = |direction: Direction, current_grid: Grid| -> Grid {
      let current_room = rooms.get_mut(&current_grid).unwrap();
      current_room.set_direction(direction);
      let distance = current_room.distance + 1;
      let next_grid = current_grid.direction(direction);
      match rooms.entry(next_grid) {
        Entry::Occupied(_) => {},
        Entry::Vacant(v) => {
          v.insert(Room::new(next_grid, direction.opposite(), distance));
        }
      }
      next_grid
    };
    for c in input.chars() {
      match c {
        '^' => continue,
        '(' => parens.push(current_grid),
        ')' => current_grid = parens.pop().unwrap(),
        '|' => {
          let last = parens.len() - 1;
          current_grid = *parens.get_mut(last).unwrap();
        },
        'N' => current_grid = grow(Direction::North, current_grid),
        'S' => current_grid = grow(Direction::South, current_grid),
        'E' => current_grid = grow(Direction::East, current_grid),
        'W' => current_grid = grow(Direction::West, current_grid),
        '$' => break,
        c => panic!(format!("Unrecognized character '{}'", c)),
      }
    }
    Rooms{rooms}
  }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Hash, Eq, Ord)]
struct Grid {
  x: i16,
  y: i16,
}

impl Grid {
  fn north(&self) -> Self {
    Grid{x: self.x, y: self.y + 1}
  }
  fn south(&self) -> Self {
    Grid{x: self.x, y: self.y - 1}
  }
  fn east(&self) -> Self {
    Grid{x: self.x + 1, y: self.y}
  }
  fn west(&self) -> Self {
    Grid{x: self.x - 1, y: self.y}
  }
  fn direction(&self, direction: Direction) -> Self {
    match direction {
      Direction::North => self.north(),
      Direction::South => self.south(),
      Direction::East => self.east(),
      Direction::West => self.west(),
      Direction::None => *self,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
  North,
  South,
  East,
  West,
  None,
}

impl Direction {
  fn opposite(&self) -> Self {
    match self {
      Direction::North => Direction::South,
      Direction::South => Direction::North,
      Direction::East => Direction::West,
      Direction::West => Direction::East,
      Direction::None => Direction::None,
    }
  }
}

#[derive(Debug, Clone)]
struct Room {
  grid: Grid,
  directions: [Direction; 4],
  direction: Direction,
  distance: usize,
}

impl Room {
  fn new(grid: Grid, direction: Direction, distance: usize) -> Self {
    let directions = match direction {
      Direction::North => [Direction::North, Direction::None, Direction::None, Direction::None],
      Direction::South => [Direction::None, Direction::South, Direction::None, Direction::None],
      Direction::East => [Direction::None, Direction::None, Direction::East, Direction::None],
      Direction::West => [Direction::None, Direction::None, Direction::None, Direction::West],
      _ => [Direction::None, Direction::None, Direction::None, Direction::None],
    };
    Room {grid, directions, direction, distance}
  }
  fn set_direction(&mut self, direction: Direction) {
    match direction {
      Direction::North => self.directions[0] = Direction::North,
      Direction::South => self.directions[1] = Direction::South,
      Direction::East => self.directions[2] = Direction::East,
      Direction::West => self.directions[3] = Direction::West,
      _ => {},
    }
  }
}


