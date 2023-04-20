#![allow(dead_code)]

pub use {false_::False, true_::True};

pub mod false_;
pub mod s;
pub mod true_;

#[derive(Debug, PartialEq)]
pub enum Node {
  False(False),
  True(True),
}
