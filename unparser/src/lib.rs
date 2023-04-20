#![allow(dead_code)]

use {crate::unparse::Unparse, parser::parse};

mod false_;
mod nil;
mod node;
mod self_;
mod true_;
pub mod unparse;

fn parse_unparse(input: &str) -> String {
  parse(input).unparse()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(parse_unparse("false"), "false");
    assert_eq!(parse_unparse("nil"), "nil");
    assert_eq!(parse_unparse("self"), "self");
    assert_eq!(parse_unparse("true"), "true");
  }
}
