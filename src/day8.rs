
use core::iter::Iterator;

struct Node {
  child_nodes: Vec<Node>,
  metadata_entries: Vec<usize>,
}

impl Node {
  fn new(numbers: &mut Iterator<Item=&usize>) -> Node {
    let node_count = *numbers.next().expect("Should have been able to read node_count");
    let metadata_count = *numbers.next().expect("Should have been able to read metadata_count");
//    println!("node_count: {}, metadata_count: {}", node_count, metadata_count);
    if metadata_count < 1 {
      panic!("Metadata count < 1.");
    }
    let mut child_nodes = Vec::with_capacity(node_count);
    for _ in 0 .. node_count {
      let node = Node::new(numbers);
      child_nodes.push(node);
    }
    let mut metadata_entries = Vec::with_capacity(metadata_count);
    for _ in 0..metadata_count {
      metadata_entries.push(*numbers.next().expect("Should have been able to read metadata"));
    }
    Node{child_nodes, metadata_entries}
  }

  fn total_metadata(&self) -> usize {
    self.child_nodes.iter().map(|n|n.total_metadata()).sum::<usize>() +
      self.metadata_entries.iter().sum::<usize>()
  }

  fn value(&self) -> usize {
    let mut value = 0;
    if self.child_nodes.len() == 0 {
      value = self.metadata_entries.iter().sum();
    } else {
      for m in self.metadata_entries.iter() {
        if *m > 0 {
          if let Some(n) = self.child_nodes.get(m - 1) {
            value += n.value();
          }
        }
      }
    }
    value
  }
}

pub fn compute(input: &[String]) {
  let numbers: Vec<usize> = input.first().unwrap().split_whitespace().map(|s|s.parse().unwrap()).collect();
  println!("{:?}", numbers.len());
  let iter = &mut numbers.iter();
  let node = Node::new(iter);
  assert_eq!(iter.next(), None);
  println!("Total metadata: {:?}", node.total_metadata());
  println!("Value: {}", node.value());
}