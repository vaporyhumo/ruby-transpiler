use ast::send::SendArg;

use {crate::unparse::Unparse, ast::Send};

impl Unparse for Send {
  fn unparse(&self) -> String {
    match &self.argument {
      Some(argument) => format!("{} {}", self.method, argument.unparse()),
      None => self.method.to_string(),
    }
  }
}

impl Unparse for SendArg {
  fn unparse(&self) -> String {
    match self {
      Self::Id(id) => id.unparse(),
      Self::Dstr(dstr) => dstr.unparse(),
    }
  }
}

#[cfg(test)]
mod tests {
  use ast::Id;

  use {
    super::*,
    ast::{s, send::Send},
  };

  #[test]
  fn test_unparse() {
    assert_eq!(s!(send, nil, "puts").unparse(), "puts")
  }
}
