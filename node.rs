pub struct Node {
  x: u64,
  y: u64,
  parent_x: u64,
  parent_y: u64,
  depth: u64,
  cost: u64,
}

impl Eq for Node {
  fn eq(&self, other: &Node) -> bool {
    self.x == other.x && self.y == other.y
  }
}

impl Ord for Node {
  fn lt(&self, other: &Node) -> bool {
    other.cost < self.cost
  }
}
