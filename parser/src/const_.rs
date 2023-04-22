use {
  crate::parse::Parse,
  ast::{Node, Const},
  lexer::Token,
};

impl Parse for Const {
  fn parse(tokens: &[Token]) -> Option<Node> {
    tokens.first().and_then(|token| match token {
      Token::Const(lexeme) => Some(Self::node(&lexeme)),
      _ => None,
    })
  }
}
