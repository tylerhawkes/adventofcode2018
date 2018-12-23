use std::cmp::Ordering;

pub fn compute(input: &[String]) {
  let tests = vec![
    convert_test(
      "#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########", 18, 1546),
    convert_test(
      "#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######",
      37,
      982,
    ),
    convert_test(
      "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######",
      46,
      859,
    ),
    convert_test(
      "#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######",
      35,
      793,
    ),
    convert_test(
      "#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######",
      54,
      536,
    ),
    convert_test(
      "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########",
      20,
      937,
    ),
  ];

  for test in tests {
    println!("Running test {:#?}", test);
    assert_eq!(compute_game(&test.0), test.1 * test.2);
  }
  compute_game(input); //>165243 <173000 <179220 <
}

fn convert_test(input: &str, rounds: i32, hit_points: i32) -> (Vec<String>, i32, i32) {
  (input.split("\n").map(|s| s.to_owned()).collect(), rounds, hit_points)
}

const WALL: char = '#';
const SPACE: char = '.';
const ELF: char = 'E';
const GOBLIN: char = 'G';

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Grid {
  x: usize,
  y: usize,
}

impl Grid {
  fn next(&self) -> [Grid; 4] {
    [
      // Up
      Grid { x: self.x, y: self.y - 1 },
      // Left
      Grid { x: self.x - 1, y: self.y },
      // Right
      Grid { x: self.x + 1, y: self.y },
      // Down
      Grid { x: self.x, y: self.y + 1 },
    ]
  }
}
// ulrd 179220
// lurd 177480
// lrud panic
// lrdu panic
// urld 186310
// ruld panic
//

#[derive(Debug, PartialEq, Clone, Eq)]
enum Team {
  Elves,
  Goblins,
}

#[derive(Debug)]
struct Player {
  team: Team,
  grid: Grid,
  hit_points: i32,
  attack_points: u32,
}

impl Player {
  fn new(grid: Grid, team: Team, hit_points: i32, attack_points: u32) -> Self {
    Player {
      team,
      grid,
      hit_points,
      attack_points,
    }
  }
}

#[derive(Debug)]
struct Game {
  //y then x
  map: Vec<Vec<char>>,
  players: Vec<Player>,
  round: usize,
}

impl Game {
  fn new(input: &[String]) -> Self {
    let mut map = vec![];
    let mut players = vec![];
    for (y, line) in input.iter().enumerate() {
      map.push(vec![]);
      for (x, c) in line.chars().enumerate() {
        match c {
          SPACE | GOBLIN | ELF | WALL => map[y].push(c),
          _ => panic!(format!("Invalid character {}", c)),
        }
        match c {
          ELF => {
            players.push(Player::new(Grid { x, y }, Team::Elves, 200, 3));
          }
          GOBLIN => {
            players.push(Player::new(Grid { x, y }, Team::Goblins, 200, 3));
          }
          _ => {}
        }
      }
    }
    Game { map, players, round: 0 }
  }
  fn sort_players(&mut self) {
    self.players.sort_by(|a, b| {
      let ord = a.grid.y.cmp(&b.grid.y);
      if ord == Ordering::Equal {
        return a.grid.x.cmp(&b.grid.x);
      }
      ord
    })
  }
  fn run_round(&mut self) -> bool {
    self.sort_players();
    self.print();
    let mut i = 0;
    println!("Starting round");
    while i < self.players.len() {
      println!("run_round {} i = {}", self.round, i);
      self.mv(i);
      let (lower_removed, remaining) = self.attack(i);
      println!("lower_removed {}, remaining {}, i {}", lower_removed, remaining, i);
      if !remaining {
        return false;
      }
      if lower_removed {
        i -= 1;
      }
      i += 1;
    }
    self.round += 1;
    true
  }
  // returns whether a lower player index was removed than the current player
  // and whether any opposing players remain
  fn attack(&mut self, player: usize) -> (bool, bool) {
    println!("player at {:?} attacking", self.players[player]);
    let team = self.players[player].team.clone();
    let mut possible_attack = None;
    let mut min_hit_points = i32::max_value();
    for n in &self.players[player].grid.next() {
      for i in 0..self.players.len() {
        // next to current player and not same team
        if &self.players[i].grid == n && self.players[i].team != team && self.players[i].hit_points < min_hit_points {
          possible_attack = Some(i);
          min_hit_points = self.players[i].hit_points;
        }
      }
    }
    if let Some(i) = possible_attack {
      println!(
        "Player {} attacking player {} ({:?} -> {:?})",
        player, i, self.players[player], self.players[i]
      );
      self.players[i].hit_points -= self.players[player].attack_points as i32;
      if self.players[i].hit_points <= 0 {
        let grid = &self.players[i].grid;
        self.map[grid.y][grid.x] = SPACE;
        println!("Removing {} player {:?}", i, self.players[i]);
        self.players.remove(i);
        return (i < player, self.players.iter().any(|p| p.team != team));
      }
    }
    (false, true)
  }

  fn mv(&mut self, player: usize) {
    println!("Player {} {:?} moving", player, &self.players[player]);
    //    self.print();
    let base = &self.players[player].grid.clone();
    let team = &self.players[player].team.clone();
    let mut checked = ::std::collections::HashSet::new();
    let mut current = Vec::with_capacity(32);
    checked.insert(base.clone());
    let n = base.next();
    for g in n.iter() {
      let (can_move, can_attack) = self.check_move_attack(g, team);
      if can_attack {
        println!("Not moving since already able to attack!");
        return;
      }
      if can_move {
        current.push(Path {
          start: g.clone(),
          end: g.clone(),
        });
        checked.insert(g.clone());
      }
    }
    let mut next = Vec::with_capacity(32);
    while !current.is_empty() {
      for Path { start, end } in current.iter() {
        match self.check_move_attack(&end, team) {
          (true, false) => {
            let n = end.next();
            n.iter().for_each(|g| {
              if !checked.contains(g) {
                next.push(Path {
                  start: start.clone(),
                  end: g.clone(),
                });
                checked.insert(g.clone());
              }
            });
          }
          (false, true) => {
            // Only move if the path end isn't right next to the beginning
            println!("current: {:?}", current);
            println!("Path found attackable opponent: {:?}, {:?} -> {:?}", base, start, end);
            if start != end {
              println!("Player {} {:?} moving to {:?} ({:?})", player, base, start, self.players[player]);
              self.players[player].grid = start.clone();
              let s = self.map[start.y][start.x];
              let b = self.map[base.y][base.x];
              self.map[base.y][base.x] = s;
              self.map[start.y][start.x] = b;
            }
            return;
          }
          _ => {}
        }
      }
      current.clear();
      next.iter().for_each(|p| current.push(p.clone()));
      next.clear();
    }
    println!("Unable to move for player {} {:?}", player, self.players[player]);
  }

  ///Returns whether the space can be moved to and if not if it can be attacked
  fn check_move_attack(&self, grid: &Grid, team: &Team) -> (bool, bool) {
    match self.map[grid.y][grid.x] {
      SPACE => (true, false),
      WALL => (false, false),
      ELF => (
        false,
        match team {
          Team::Goblins => true,
          _ => false,
        },
      ),
      GOBLIN => (
        false,
        match team {
          Team::Elves => true,
          _ => false,
        },
      ),
      c => panic!(format!("Disallowed character '{}' in the map!", c)),
    }
  }

  fn print(&self) {
    self.map.iter().for_each(|v| {
      v.iter().for_each(|c| print!("{}", c));
      println!();
    });
    self.players.iter().for_each(|p| println!("{:?}", p));
  }
}

#[derive(Debug, Clone)]
struct Path {
  start: Grid,
  end: Grid,
}

fn compute_game(input: &[String]) -> i32 {
  let mut game = Game::new(input);
  while game.run_round() {}
  game.print();
  let hit_points = game.players.iter().map(|p| p.hit_points).sum::<i32>();
  println!("{}, {}, {}", game.round, hit_points, game.round as i32 * hit_points);
  game.round as i32 * hit_points
}
