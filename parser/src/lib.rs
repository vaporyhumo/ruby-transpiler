use {
  ast::{
    Begin, Const, Dstr, False, Global, Int, Nil, Node, Self_, Send, Symbol,
    True,
  },
  lexer::Token,
  parse::Parse,
};

mod begin;
mod const_;
mod dstr;
mod false_;
mod global;
mod int;
mod nil;
mod parse;
mod self_;
mod send;
mod symbol;
mod true_;

fn is_whitespace(token: &Token) -> bool {
  match token {
    Token::WSpace(_) => false,
    _ => true,
  }
}

pub fn parse(string: &str) -> Node {
  let tokens: Vec<Token> = Token::lex(string);
  let tokens: Vec<Token> =
    tokens.into_iter().filter(|t| is_whitespace(&t)).collect();
  False::parse(&tokens)
    .or(Const::parse(&tokens))
    .or(Global::parse(&tokens))
    .or(Int::parse(&tokens))
    .or(True::parse(&tokens))
    .or(Nil::parse(&tokens))
    .or(Self_::parse(&tokens))
    .or(Symbol::parse(&tokens))
    .or(Dstr::parse(&tokens))
    .or(Begin::parse(&tokens))
    .or(Send::parse(&tokens))
    .expect(format!("Unexpected token: {:?}", tokens).as_str())
}

#[cfg(test)]
mod tests {
  use {
    super::*,
    ast::{s, Id},
  };

  #[test]
  fn test_parse() {
    assert_eq!(parse("1234567890"), s!(int, "1234567890").into());
    assert_eq!(parse("-1234567890"), s!(int, "-1234567890").into());
    assert_eq!(parse("false"), s!(false).into());
    assert_eq!(parse("nil"), s!(nil).into());
    assert_eq!(parse("self"), s!(self).into());
    assert_eq!(parse(":sym"), s!(sym, ":sym").into());
    assert_eq!(parse("true"), s!(true).into());
    assert_eq!(parse(" "), s!(begin).into());
    assert_eq!(parse("puts"), s!(send, nil, "puts").into());
    assert_eq!(
      parse("puts \"hello world\""),
      s!(send, nil, "puts", s!(dstr, "\"hello world\"")).into()
    );
  }
}
