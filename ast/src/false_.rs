use crate::Node;

#[derive(Debug, PartialEq)]
pub struct False {}

impl False {
  pub fn new() -> Self {
    Self {}
  }

  pub fn node() -> Node {
    Node::False(Self::new())
  }
}

impl Default for False {
  fn default() -> Self {
    Self::new()
  }
}

impl From<False> for Node {
  fn from(_: False) -> Self {
    False::node()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_false_new() {
    assert_eq!(False::new(), False {});
  }

  #[test]
  fn test_false_node() {
    assert_eq!(False::node(), Node::False(False {}));
  }

  #[test]
  fn test_false_default() {
    assert_eq!(False::default(), False {});
  }

  #[test]
  fn test_false_from() {
    assert_eq!(Node::from(False::new()), Node::False(False {}));
  }
}
