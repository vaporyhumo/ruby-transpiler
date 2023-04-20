use {false_::False, true_::True};

mod false_;
mod split;
mod true_;

#[derive(Debug, PartialEq)]
pub enum Token {
  False,
  True,
}

impl Token {
  fn lex_token(string: &str) -> Option<(Token, String)> {
    False::lex(string).or(True::lex(string))
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
  use super::*;

  #[test]
  fn test_lex() {
    assert_eq!(Token::lex("false"), vec![Token::False]);
    assert_eq!(Token::lex("true"), vec![Token::True]);
  }
}
