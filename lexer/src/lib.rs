use {
  const_::Const, dstring::DString, false_::False, id::Id, int::Int, nil::Nil, self_::Self_, symbol::Symbol, true_::True,
  wspace::WSpace,
};

mod const_;
mod dstring;
mod false_;
mod id;
mod int;
mod nil;
mod self_;
mod split;
mod symbol;
mod true_;
mod wspace;

#[derive(Debug, PartialEq)]
pub enum Token {
  Const(String),
  DString(String),
  False,
  Id(String),
  Int(String),
  Nil,
  Self_,
  Symbol(String),
  True,
  WSpace(String),
}

impl Token {
  fn lex_token(string: &str) -> Option<(Token, String)> {
    WSpace::lex(string)
      .or(Const::lex(string))
      .or(Int::lex(string))
      .or(False::lex(string))
      .or(True::lex(string))
      .or(Nil::lex(string))
      .or(Self_::lex(string))
      .or(Symbol::lex(string))
      .or(Id::lex(string))
      .or(DString::lex(string))
  }

  pub fn lex(string: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut string: String = string.to_string();

    loop {
      if string.is_empty() {
        return tokens;
      }
      match Self::lex_token(&string) {
        Some((token, rest)) => {
          tokens.push(token);
          string = rest;
        }
        None => panic!("Unexpected token: {}", string),
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use {
    super::*,
  };


  #[test]
  fn test_lex() {
    let lex = Token::lex;
    assert_eq!(lex("1234567890"), vec![Int::token("1234567890")]);
    assert_eq!(lex("-1234567890"), vec![Int::token("-1234567890")]);
    assert_eq!(lex("false"), vec![False::token()]);
    assert_eq!(lex("nil"), vec![Nil::token()]);
    assert_eq!(lex("true"), vec![True::token()]);
    assert_eq!(lex("self"), vec![Self_::token()]);
    assert_eq!(lex(":sym"), vec![Symbol::token(":sym")]);
    assert_eq!(lex("hello"), vec![Id::token("hello")]);
    assert_eq!(lex("\"hello\""), vec![DString::token("\"hello\"")]);
    assert_eq!(lex("puts \"hello\""), vec![Id::token("puts"), WSpace::token(" "), DString::token("\"hello\"")]);
    assert_eq!(lex(" "), vec![WSpace::token(" ")]);
  }
}
