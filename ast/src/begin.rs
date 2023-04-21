use crate::Node;

#[derive(Debug, PartialEq)]
pub struct Begin {}

impl Begin {
  pub fn new() -> Self {
    Self {}
  }

  pub fn node() -> Node {
    Node::Begin(Self::new())
  }
}

impl Default for Begin {
  fn default() -> Self {
    Self::new()
  }
}

impl From<Begin> for Node {
  fn from(_: Begin) -> Self {
    Begin::node()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_false_new() {
    assert_eq!(Begin::new(), Begin {});
  }

  #[test]
  fn test_false_node() {
    assert_eq!(Begin::node(), Node::Begin(Begin {}));
  }

  #[test]
  fn test_false_default() {
    assert_eq!(Begin::default(), Begin {});
  }

  #[test]
  fn test_false_from() {
    assert_eq!(Node::from(Begin::new()), Node::Begin(Begin {}));
  }
}
