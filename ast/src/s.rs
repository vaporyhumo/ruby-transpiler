#[allow(unused_macros)]
#[macro_export]
macro_rules! s {
  (false) => {
    False::new()
  };
}

#[cfg(test)]
mod tests {
  use crate::false_::False;

  #[test]
  fn test_false() {
    assert_eq!(s!(false), False {});
  }
}
