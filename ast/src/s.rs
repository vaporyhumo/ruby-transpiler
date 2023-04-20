#[allow(unused_macros)]
#[macro_export]
macro_rules! s {
  (false) => {
    False::new()
  };
  (true) => {
    True::new()
  };
}

#[cfg(test)]
mod tests {
  use crate::{false_::False, true_::True};

  #[test]
  fn test_false() {
    assert_eq!(s!(false), False {});
  }

  #[test]
  fn test_true() {
    assert_eq!(s!(true), True {});
  }
}
