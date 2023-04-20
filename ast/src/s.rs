#[allow(unused_macros)]
#[macro_export]
macro_rules! s {
  (false) => {
    False::new()
  };
  (nil) => {
    Nil::new()
  };
  (self) => {
    Self_::new()
  };
  (true) => {
    True::new()
  };
}

#[cfg(test)]
mod tests {
  use crate::{false_::False, nil::Nil, self_::Self_, true_::True};

  #[test]
  fn test_s() {
    assert_eq!(s!(false), False {});
    assert_eq!(s!(nil), Nil {});
    assert_eq!(s!(self), Self_ {});
    assert_eq!(s!(true), True {});
  }
}
