use {
  ast::{False, Node, True},
  lexer::Token,
  parse::Parse,
};

mod false_;
mod parse;
mod true_;

pub fn parse(string: &str) -> Node {
  let tokens: Vec<Token> = Token::lex(string);
  False::parse(&tokens).or(True::parse(&tokens)).unwrap()
}

#[cfg(test)]
mod tests {
  use {super::*, ast::s};

  #[test]
  fn test_false() {
    assert_eq!(parse("false"), s!(false).into(),);
  }
}
