#[allow(unused_macros)]
#[macro_export]
macro_rules! s {
  (false) => {
    False::new()
  };
  (nil) => {
    Nil::new()
  };
  (true) => {
    True::new()
  };
}

#[cfg(test)]
mod tests {
  use crate::{false_::False, nil::Nil, true_::True};

  #[test]
  fn test_s() {
    assert_eq!(s!(false), False {});
    assert_eq!(s!(nil), Nil {});
    assert_eq!(s!(true), True {});
  }
}
