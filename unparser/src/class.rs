use {crate::unparse::Unparse, ast::Class};

impl Unparse for Class {
  fn unparse(&self) -> String {
    format!("class {}\nend", self.name.unparse())
  }
}

#[cfg(test)]
mod tests {
  use {
    super::*,
    ast::{const_::Const, class::Class, s},
  };

  #[test]
  fn test_unparse() {
    assert_eq!(s!(class s!(const "Foo"), nil).unparse(), "class Foo\nend");
  }
}
