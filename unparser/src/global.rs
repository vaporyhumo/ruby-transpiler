use {crate::unparse::Unparse, ast::Global};

impl Unparse for Global {
  fn unparse(&self) -> String {
    self.lexeme.to_string()
  }
}

#[cfg(test)]
mod tests {
  use {
    super::*,
    ast::{s, global::Global},
  };

  #[test]
  fn test_unparse() {
    assert_eq!(s!(global "$foo").unparse(), "$foo")
  }
}
