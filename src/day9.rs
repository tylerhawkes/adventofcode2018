use crate::day9::double::*;
use std::fmt::Debug;
use std::fmt::{Display, Error, Formatter};

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
  id: u32,
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

pub fn compute(input: &[String]) {
//    let mut success = vec![];
//    success.push(run_game(10, 1618) == 8317);
//    success.push(run_game(13, 7999) == 146373);
//    success.push(run_game(17, 1104) == 2764);
//    success.push(run_game(21, 6111) == 54718);
//    success.push(run_game(30, 5807) == 37305);
//    println!("{:?}", success);
  run_game(418, 70769); //402398
  run_game(418, 70769*100); //3426843186
}

struct Circle {
  circle: List<Marble>,
  current_index: Pointer,
}

impl Circle {
  fn place_marble(&mut self, marble: &Marble, player: &mut Player) -> usize {
    let mut score_increase = 0;
    if marble.id % 23 == 0 {
      //handle special case
      for _ in 0..7 {
        self.current_index = self.circle.backward(self.current_index);
      }
      let removal = self.current_index.clone();
      self.current_index = self.circle.forward(self.current_index);
      let removed_marble = self.circle.remove(removal);
      let score_increase = marble.id
        + removed_marble.id;
//      println!("score_increase: {}", score_increase);
        player.score += score_increase;
    } else {
      self.current_index = self.circle.forward(self.current_index);
      self.current_index = self.circle.insert_after(self.current_index, marble.clone());
    }
    //    println!("{} ({:?}): {:?}", self.current_index, self.circle[self.current_index], self.circle);
    score_increase
  }
}

pub fn run_game(player_count: usize, last_marble: u32) -> u32 {
  let mut players = vec![];
  for _ in 0..player_count {
    players.push(Player { score: 0 });
  }
  let mut circle_list = double::List::new();
  //let marbles = vec![0,  2,  1].into_iter().map(|id|Marble{id}).for_each(|m|circle_list.push_back(m));
  circle_list.push_back(Marble { id: 0 });
  let current = circle_list.push_back(Marble { id: 2 });
  circle_list.push_back(Marble { id: 1 });
  let mut circle = Circle {
    circle: circle_list,
    current_index: current,
  };
  let mut current_marble = Marble { id: 2 };
  let mut players = Players {
    players,
    current_player: 0,
  };
  //  let mut score_increases = vec![];
  for i in current_marble.id..=last_marble {
    let marble = current_marble.next();
    if marble.id % 10000 == 0 {
      println!("On id {}", marble.id);
    }
    circle.place_marble(marble, players.next());
  }
  //  score_increases.sort();
  //  score_increases.iter().for_each(|s|println!("{}", s));
  let max_score = players.players.iter().map(|p| p.score).max().unwrap();
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

mod double {
  use slab::Slab;
  use std::fmt;
  use std::ops::{Index, IndexMut};

  fn main() {
    println!("create an empty doubly-linked list");
    let mut list = List::new();
    println!("{:?}\n", list);

    println!("push 9 to the back");
    let a = list.push_back(9);
    println!("{:?}\n", list);

    println!("push 0 to the front");
    let b = list.push_front(0);
    println!("{:?}\n", list);

    println!("insert 3 after {}", list[a].value);
    let c = list.insert_after(a, 3);
    println!("{:?}\n", list);

    println!("change {} to 1", list[a].value);
    list[a].value = 1;
    println!("{:?}\n", list);

    println!("insert 2 before {}", list[c].value);
    let d = list.insert_before(c, 2);
    println!("{:?}\n", list);

    println!("remove {}", list.remove(a));
    println!("{:?}\n", list);

    println!("remove {}", list.remove(d));
    println!("{:?}\n", list);

    println!("remove {}", list.remove(b));
    println!("{:?}\n", list);

    println!("remove {}", list.remove(c));
    println!("{:?}\n", list);
  }

  // A doubly linked list.
  pub struct List<T> {
    // All nodes get stored into this slab. A slab is basically just a
    // `Vec<Option<T>>` in disguise. We use it as a simple node allocator.
    slab: Slab<Node<T>>,
    // The head of the doubly linked list.
    head: Pointer,
    // The tail of the doubly linked list.
    tail: Pointer,
  }

  // A node in a doubly-linked list.
  pub struct Node<T> {
    // The value stored in this node.
    value: T,
    // The next node in the list.
    next: Pointer,
    // The previous node in the list.
    prev: Pointer,
  }

  // A `Pointer` is just an index that refers to a node in the slab.
  #[derive(Eq, PartialEq, Copy, Clone)]
  pub struct Pointer(usize);

  impl Pointer {
    // The null pointer is `!0`, which is the largest possible value of type
    // `usize`. There's no way we'll ever have a legitimate index that large.
    #[inline]
    fn null() -> Pointer {
      Pointer(!0)
    }

    // Returns `true` if this pointer is null.
    #[inline]
    pub fn is_null(&self) -> bool {
      *self == Pointer::null()
    }
  }

  // Just for convenience, so that we can type `self[i]` instead of `self.slab[i]`.
  impl<T> Index<Pointer> for List<T> {
    type Output = Node<T>;

    fn index(&self, index: Pointer) -> &Node<T> {
      &self.slab[index.0]
    }
  }

  // Just for convenience, so that we can type `self[i]` instead of `self.slab[i]`.
  impl<T> IndexMut<Pointer> for List<T> {
    fn index_mut(&mut self, index: Pointer) -> &mut Node<T> {
      &mut self.slab[index.0]
    }
  }

  impl<T> List<T> {
    // Returns a new doubly linked list.
    pub fn new() -> List<T> {
      List {
        slab: Slab::new(),
        head: Pointer::null(),
        tail: Pointer::null(),
      }
    }

    pub fn reserve(&mut self, additional: usize) {
      self.slab.reserve(additional);
    }

    pub fn next(&self, node: Pointer) -> Pointer {
      self[node].next
    }

    pub fn prev(&self, node: Pointer) -> Pointer {
      self[node].prev
    }

    pub fn forward(&self, node: Pointer) -> Pointer {
      let next = self[node].next;
      if next.is_null() {
        return self.head;
      }
      next
    }

    pub fn backward(&self, node: Pointer) -> Pointer {
      let prev = self[node].prev;
      if prev.is_null() {
        return self.tail;
      }
      prev
    }

    pub fn head(&self) -> Pointer {
      self.head
    }

    pub fn tail(&self) -> Pointer {
      self.tail
    }

    // Inserts a new element at the back of the list.
    pub fn push_back(&mut self, t: T) -> Pointer {
      let tail = self.tail;
      if tail.is_null() {
        let n = Pointer(self.slab.insert(Node {
          value: t,
          prev: Pointer::null(),
          next: Pointer::null(),
        }));
        self.head = n;
        self.tail = n;
        n
      } else {
        self.insert_after(tail, t)
      }
    }

    // Inserts a new element at the front of the list.
    pub fn push_front(&mut self, t: T) -> Pointer {
      let head = self.head;
      if head.is_null() {
        self.push_back(t)
      } else {
        self.insert_before(head, t)
      }
    }

    // Inserts a new element after `node`.
    pub fn insert_after(&mut self, node: Pointer, t: T) -> Pointer {
      let next = self[node].next;
      let n = Pointer(self.slab.insert(Node {
        value: t,
        prev: node,
        next,
      }));

      if next.is_null() {
        self.tail = n;
      } else {
        self[next].prev = n;
      }
      self[node].next = n;
      n
    }

    // Inserts a new element before `node`.
    pub fn insert_before(&mut self, node: Pointer, t: T) -> Pointer {
      let prev = self[node].prev;
      let n = Pointer(self.slab.insert(Node {
        value: t,
        prev,
        next: node,
      }));

      if prev.is_null() {
        self.head = n;
      } else {
        self[prev].next = n;
      }
      self[node].prev = n;
      n
    }

    // Removes `node` from the list and returns its value.
    pub fn remove(&mut self, node: Pointer) -> &T {
      let prev = self[node].prev;
      let next = self[node].next;

      if prev.is_null() {
        self.head = next;
      } else {
        self[prev].next = next;
      }

      if next.is_null() {
        self.tail = prev;
      } else {
        self[next].prev = prev;
      }
      //self.slab.remove(node.0).value
      &self.slab[node.0].value
    }
  }

  impl<T: fmt::Debug> fmt::Debug for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let mut first = true;
      let mut n = self.head;

      write!(f, "List(")?;
      while !n.is_null() {
        if !first {
          write!(f, ", ")?;
        }
        first = false;

        write!(f, "{:?}", self[n].value)?;
        n = self[n].next;
      }
      write!(f, ")")?;

      Ok(())
    }
  }
}
