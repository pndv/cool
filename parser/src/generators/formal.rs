use crate::model::formal::Formal;
use crate::model::{Ident, Type};
use lexer::iter::token::{BaseTokenIter, BufferedTokenIter};
use lexer::model::constants::{COLON_TYPE, COMMA_TYPE, IDENT_TYPE};
use lexer::model::token::Token;

/// Formals |-> formal {{, formals}}
pub(super) fn gen_formals(iter: &mut BufferedTokenIter) -> Result<Vec<Formal>, String> {
  let mut formals: Vec<Formal> = Vec::new();
  let mut formal = gen_formal(iter)?;
  formals.push(formal);

  while iter.has_next() && iter.peek_eq(&COMMA_TYPE) {
    assert!(iter.consume_required(&COMMA_TYPE).is_ok()); // Consume ','

    formal = gen_formal(iter)?;
    formals.push(formal);
  }

  Ok(formals)
}

/// Formal |-> ID : TYPE
fn gen_formal(token_iter: &mut BufferedTokenIter) -> Result<Formal, String> {
  let Token::Ident { value, .. } = token_iter.get_required(&IDENT_TYPE)? else { unreachable!() };
  let formal_name = Ident::from(value);

  token_iter.consume_required(&COLON_TYPE)?; // consume colon

  let Token::Ident { value, .. } = token_iter.get_required(&IDENT_TYPE)? else { unreachable!() };
  let formal_type: Type = Type::from(value);

  Ok((formal_name, formal_type).into())
}
