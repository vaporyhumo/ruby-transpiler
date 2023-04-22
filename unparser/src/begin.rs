use {crate::unparse::Unparse, ast::Begin};

impl Unparse for Begin {
  fn unparse(&self) -> String {
    String::new()
  }
}

#[cfg(test)]
mod tests {
  use {
    super::*,
    ast::{begin::Begin, s},
  };

  #[test]
  fn test_unparse() {
    assert_eq!(s!(begin).unparse(), "");
  }
}
