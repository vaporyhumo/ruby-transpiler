#![allow(dead_code)]

pub use false_::False;

pub mod false_;
pub mod s;

#[derive(Debug, PartialEq)]
pub enum Node {
  False(False),
}
