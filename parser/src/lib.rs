use {
  ast::{False, Nil, Node, True},
  lexer::Token,
  parse::Parse,
};

mod false_;
mod nil;
mod parse;
mod true_;

pub fn parse(string: &str) -> Node {
  let tokens: Vec<Token> = Token::lex(string);
  False::parse(&tokens).or(True::parse(&tokens)).or(Nil::parse(&tokens)).unwrap()
}

#[cfg(test)]
mod tests {
  use {super::*, ast::s};

  #[test]
  fn test_parse() {
    assert_eq!(parse("false"), s!(false).into(),);
    assert_eq!(parse("nil"), s!(nil).into(),);
    assert_eq!(parse("true"), s!(true).into(),);
  }
}
