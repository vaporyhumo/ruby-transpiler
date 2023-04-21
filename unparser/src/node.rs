use {crate::unparse::Unparse, ast::Node};

impl Unparse for Node {
  fn unparse(&self) -> String {
    match self {
      Node::Begin(begin) => begin.unparse(),
      Node::Dstr(dstr) => dstr.unparse(),
      Node::False(false_) => false_.unparse(),
      Node::Id(id) => id.unparse(),
      Node::Nil(nil) => nil.unparse(),
      Node::Self_(self_) => self_.unparse(),
      Node::Send(send) => send.unparse(),
      Node::True(true_) => true_.unparse(),
    }
  }
}
