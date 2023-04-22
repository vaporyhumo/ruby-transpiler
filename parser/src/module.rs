use ast::Const;

use {
  crate::parse::Parse,
  ast::{Module, Node},
  lexer::Token,
};

impl Parse for Module {
  fn parse(tokens: &[Token]) -> Option<Node> {
    parse_module_token(tokens).and_then(|(_, tokens)| {
      parse_constant_token(tokens).and_then(|(name, tokens)| {
        parse_end_token(tokens)
          .and_then(|(_, _)| Some(Node::Module(Module { name })))
      })
    })
  }
}

fn parse_module_token(tokens: &[Token]) -> Option<(Token, &[Token])> {
  let (token, tokens) = tokens.split_first()?;
  match token {
    Token::Module => Some((token.clone(), tokens)),
    _ => None,
  }
}

fn parse_constant_token(tokens: &[Token]) -> Option<(Const, &[Token])> {
  let (token, tokens) = tokens.split_first()?;
  match token {
    Token::Const(name) => Some((Const::new(name), tokens)),
    _ => None,
  }
}

fn parse_end_token(tokens: &[Token]) -> Option<(Token, &[Token])> {
  let (token, tokens) = tokens.split_first()?;
  match token {
    Token::End => Some((token.clone(), tokens)),
    _ => None,
  }
}
