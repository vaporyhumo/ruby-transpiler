use {crate::unparse::Unparse, ast::Node};

impl Unparse for Node {
  fn unparse(&self) -> String {
    match self {
      Node::False(false_) => false_.unparse(),
    }
  }
}
