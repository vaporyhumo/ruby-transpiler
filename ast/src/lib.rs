#![allow(dead_code)]

pub use {false_::False, nil::Nil, self_::Self_, true_::True};

pub mod false_;
pub mod nil;
pub mod s;
pub mod self_;
pub mod true_;

#[derive(Debug, PartialEq)]
pub enum Node {
  False(False),
  Nil(Nil),
  Self_(Self_),
  True(True),
}
