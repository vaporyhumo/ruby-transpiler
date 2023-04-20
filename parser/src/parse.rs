use ast::Node;
use lexer::Token;

pub trait Parse {
  fn parse(tokens: &[Token]) -> Option<Node>;
}
