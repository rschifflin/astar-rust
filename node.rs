use std::cast;
pub struct Node<'a> {
  x: u64,
  y: u64,
  depth: u64,
  cost: u64,
  parent: Option<&'a Node<'a>>
}

impl<'a> Eq for Node<'a> {
  fn eq(&self, other: &Node) -> bool {
    self.x == other.x && self.y == other.y
  }
}

impl<'a> Ord for Node<'a> {
  fn lt(&self, other: &Node) -> bool {
    other.cost < self.cost
  }
}

pub struct UnsafeNodeList<'a> {
  nodes: ~[~Node<'a>]
}

impl<'b> UnsafeNodeList<'b> {
  pub fn push(&self, node: ~Node<'b>) {
    unsafe {
      cast::transmute_mut(self).nodes.push(node);
    }
  }

  pub fn last<'a>(&'b self) -> &'b Node<'b> {
    &*self.nodes[self.nodes.len() - 1]
  }
}
