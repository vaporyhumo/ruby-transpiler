use {
  crate::parse::Parse,
  ast::{False, Node},
  lexer::Token,
};

impl Parse for False {
  fn parse(tokens: &[Token]) -> Option<Node> {
    tokens.first().map(|token| match token {
      Token::False => Self::node(),
    })
  }
}
