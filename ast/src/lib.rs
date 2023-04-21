#![allow(dead_code)]

pub use {
  begin::Begin, dstr::Dstr, false_::False, id::Id, nil::Nil, self_::Self_,
  send::Send, symbol::Symbol, true_::True,
};

pub mod begin;
pub mod dstr;
pub mod false_;
pub mod id;
pub mod nil;
pub mod s;
pub mod self_;
pub mod send;
pub mod symbol;
pub mod true_;

#[derive(Debug, PartialEq)]
pub enum Node {
  Begin(Begin),
  Dstr(Dstr),
  False(False),
  Id(Id),
  Nil(Nil),
  Self_(Self_),
  Send(Send),
  Symbol(Symbol),
  True(True),
}
