#![allow(dead_code)]

pub use {false_::False, nil::Nil, true_::True};

pub mod false_;
pub mod nil;
pub mod s;
pub mod true_;

#[derive(Debug, PartialEq)]
pub enum Node {
  False(False),
  Nil(Nil),
  True(True),
}
