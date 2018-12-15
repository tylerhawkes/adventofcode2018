use std::fmt::{Display, Formatter, Error};
use std::fmt::Debug;
use ixlist::*;


#[derive(Debug, Copy, Clone)]
struct Player {
  score: u32,
}

struct Players {
  players: Vec<Player>,
  current_player: usize,
}

impl Players {
  fn next(&mut self) -> &mut Player {
    self.current_player = step(self.current_player, 1, self.players.len());
    &mut self.players[self.current_player]
  }
}

#[derive(Copy, Clone)]
struct Marble {
  id: u32
}

impl Display for Marble {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "{}", self.id)?;
    Ok(())
  }
}

impl Debug for Marble {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "{}", self.id)?;
    Ok(())
  }
}

impl Marble {
  fn next(&mut self) -> &Self {
    self.id += 1;
    self
  }
}

struct Circle {
  circle: Vec<Marble>,
  current_index: usize,
}

pub fn compute(input: &[String]) {
//  let mut success = vec![];
//  success.push(run_game(10, 1618) == 8317);
//  success.push(run_game(13, 7999) == 146373);
//  success.push(run_game(17, 1104) == 2764);
//  success.push(run_game(21, 6111) == 54718);
//  success.push(run_game(30, 5807) == 37305);
//  println!("{:?}", success);
  run_game(418, 70769);
//  run_game(418, 70769*100);
}

impl Circle {
  fn place_marble(&mut self, marble: &Marble, player: &mut Player) -> usize {
    let len = self.circle.len();
    let mut score_increase = 0;
    if marble.id % 23 == 0 {
      //handle special case
      self.current_index = step(self.current_index, -7, len);
      let score_increase = marble.id + self.circle.remove(self.current_index).id;
//      println!("score_increase: {}", score_increase);
      player.score += score_increase;
    } else {
      self.current_index = step(self.current_index, 2, len+1);
      if self.current_index == 0 {
        self.current_index = 1;
      }
      self.circle.insert(self.current_index, marble.clone());
    }
//    println!("{} ({:?}): {:?}", self.current_index, self.circle[self.current_index], self.circle);
    score_increase
  }
}

pub fn run_game(player_count: usize, last_marble: u32) -> u32 {
  let mut players = vec![];
  for _ in 0..player_count {
    players.push(Player { score: 0});
  }
  let mut circle = Circle{circle: vec![0,  2,  1].into_iter().map(|id|Marble{id}).collect(), current_index: 1};
  circle.circle.reserve(last_marble as usize);
  let mut current_marble = Marble{id: 2};
  let mut players = Players{players, current_player: 0};
//  let mut score_increases = vec![];
  for i in current_marble.id..=last_marble {
    let marble = current_marble.next();
    if i % 1000 == 0 {
      println!("On id {}", marble.id);
    }
    circle.place_marble(marble, players.next());
//    let score_increase = circle.place_marble(marble, players.next());
//    if score_increase > 0 {
//      score_increases.push(score_increase);
//    }
//    if score_increase == last_marble_points || score_increase > last_marble_points * 2 {
//      println!("Hit exact score...breaking");
//      break;
//    }
  }
//  score_increases.sort();
//  score_increases.iter().for_each(|s|println!("{}", s));
  let max_score = players.players.iter().map(|p|p.score).max().unwrap();
  println!("max score: {}", &max_score);
  max_score
}

fn step(start: usize, step: isize, len: usize) -> usize {
  let i = start as isize + step;
  let len = len as isize;
  let v = (((i as isize % len) + len) % len) as usize;
//  println!("{}, {}, {}, {}", start, step, len, v);
  v
}

#[test]
fn test_step() {
  assert_eq!(step(5, 2, 8), 7);
  assert_eq!(step(5, 4, 8), 1);
  assert_eq!(step(3, -5, 8), 6);
  assert_eq!(step(20, 16, 8), 4);
  assert_eq!(step(5, 2, 7), 0);
}