use crate::nodes::{Id, Type};
use crate::tokens::{consume_required, match_required_token, FilteredTokensIterator, COLON_TYPE, COMMA_TYPE, IDENT_TYPE};

/// Formals |-> formal {{, formals}}
pub fn gen_formals(token_iter: &mut FilteredTokensIterator) -> Vec<Formal> {
  let mut formals: Vec<Formal> = Vec::new();
  let mut formal = gen_formal(token_iter);
  formals.push(formal);

  while token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&COMMA_TYPE) {
    consume_required(token_iter, COMMA_TYPE); // Consume ','

    formal = gen_formal(token_iter);
    formals.push(formal);
  }

  formals
}

/// Formal |-> ID : TYPE
fn gen_formal(token_iter: &mut FilteredTokensIterator) -> Formal {
  let mut token = match_required_token(token_iter.next(), IDENT_TYPE);
  let formal_name: Id = Id::from(token);

  consume_required(token_iter, COLON_TYPE); // consume colon

  token = match_required_token(token_iter.next(), IDENT_TYPE);
  let formal_type: Type = Type::from(token);

  (formal_name, formal_type).into()
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Formal {
  pub(crate) formal_name: Id,
  pub(crate) formal_type: Type,
}

impl From<(Id, Type)> for Formal {
  fn from((formal_name, formal_type): (Id, Type)) -> Self {
    Self {
      formal_name,
      formal_type,
    }
  }
}