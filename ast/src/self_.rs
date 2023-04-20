use crate::Node;

#[derive(Debug, PartialEq)]
pub struct Self_ {}

impl Self_ {
  pub fn new() -> Self {
    Self {}
  }

  pub fn node() -> Node {
    Node::Self_(Self::new())
  }
}

impl Default for Self_ {
  fn default() -> Self {
    Self::new()
  }
}

impl From<Self_> for Node {
  fn from(_: Self_) -> Self {
    Self_::node()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_self_new() {
    assert_eq!(Self_::new(), Self_ {});
  }

  #[test]
  fn test_self_node() {
    assert_eq!(Self_::node(), Node::Self_(Self_ {}));
  }

  #[test]
  fn test_self_default() {
    assert_eq!(Self_::default(), Self_ {});
  }

  #[test]
  fn test_self_from() {
    assert_eq!(Node::from(Self_::new()), Node::Self_(Self_ {}));
  }
}
