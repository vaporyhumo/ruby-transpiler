use crate::Node;

#[derive(Debug, PartialEq)]
pub struct Nil {}

impl Nil {
  pub fn new() -> Self {
    Self {}
  }

  pub fn node() -> Node {
    Node::Nil(Self::new())
  }
}

impl Default for Nil {
  fn default() -> Self {
    Self::new()
  }
}

impl From<Nil> for Node {
  fn from(_: Nil) -> Self {
    Nil::node()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_nil_new() {
    assert_eq!(Nil::new(), Nil {});
  }

  #[test]
  fn test_nil_node() {
    assert_eq!(Nil::node(), Node::Nil(Nil {}));
  }

  #[test]
  fn test_nil_default() {
    assert_eq!(Nil::default(), Nil {});
  }

  #[test]
  fn test_nil_from() {
    assert_eq!(Node::from(Nil::new()), Node::Nil(Nil {}));
  }
}
