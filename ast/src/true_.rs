use crate::Node;

#[derive(Debug, PartialEq)]
pub struct True {}

impl True {
  pub fn new() -> Self {
    Self {}
  }

  pub fn node() -> Node {
    Node::True(Self::new())
  }
}

impl Default for True {
  fn default() -> Self {
    Self::new()
  }
}

impl From<True> for Node {
  fn from(_: True) -> Self {
    True::node()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_true_new() {
    assert_eq!(True::new(), True {});
  }

  #[test]
  fn test_true_node() {
    assert_eq!(True::node(), Node::True(True {}));
  }

  #[test]
  fn test_true_default() {
    assert_eq!(True::default(), True {});
  }

  #[test]
  fn test_true_from() {
    assert_eq!(Node::from(True::new()), Node::True(True {}));
  }
}
