use {
  crate::parse::Parse,
  ast::{Begin, Node},
  lexer::Token,
};

impl Parse for Begin {
  fn parse(tokens: &[Token]) -> Option<Node> {
    match tokens {
      [] => Some(Self::node()),
      _ => None,
    }
  }
}
