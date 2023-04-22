#![allow(dead_code)]

use {crate::unparse::Unparse, parser::parse};

mod begin;
mod const_;
mod dstr;
mod false_;
mod id;
mod int;
mod nil;
mod node;
mod self_;
mod send;
mod symbol;
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
    assert_eq!(parse_unparse("1234567890"), "1234567890");
    assert_eq!(parse_unparse("-1234567890"), "-1234567890");
    assert_eq!(parse_unparse("false"), "false");
    assert_eq!(parse_unparse("foo"), "foo");
    assert_eq!(parse_unparse("Foo"), "Foo");
    assert_eq!(parse_unparse("nil"), "nil");
    assert_eq!(parse_unparse("self"), "self");
    assert_eq!(parse_unparse(":sym"), ":sym");
    assert_eq!(parse_unparse("true"), "true");
    assert_eq!(parse_unparse("hello \"world\""), "hello \"world\"");
    assert_eq!(parse_unparse("puts \"hello world\""), "puts \"hello world\"");
  }
}
