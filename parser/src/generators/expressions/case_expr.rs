use crate::generators::expressions::gen_expression;
use crate::model::expressions::{CaseBranch, Expression};
use crate::model::{Ident, Type};
use lexer::iter::token::{BaseTokenIter, BufferedTokenIter};
use lexer::model::constants::{CASE_BRANCH_TYPE, CASE_TYPE, COLON_TYPE, END_CASE_TYPE, IDENT_TYPE, OF_TYPE, SEMI_COLON_TYPE};
use lexer::model::token::Token;

pub(super) fn gen_case_expression(iter: &mut BufferedTokenIter) -> Result<Expression, String> {
  iter.consume_required(&CASE_TYPE)?;

  let predicate_expr = gen_expression(iter, &OF_TYPE)?;
  iter.consume_required(&OF_TYPE)?;

  let branches = gen_case_branch_list(iter)?;

  iter.consume_required(&END_CASE_TYPE)?;

  Ok(Expression::Case {
    switch_expression: Box::from(predicate_expr),
    branches,
  })
}

/// `ID` : `TYPE` => `expr` ; {{ `ID` : `TYPE` => `expr` ; }}
fn gen_case_branch_list(iter: &mut BufferedTokenIter) -> Result<Vec<CaseBranch>, String> {
  let mut case_branch_list: Vec<CaseBranch> = Vec::new();

  while iter.has_next() && !iter.peek_eq(&END_CASE_TYPE) {
    case_branch_list.push(gen_case_branch(iter)?);
  }

  assert!(!case_branch_list.is_empty()); // case expression must have at least one branch

  Ok(case_branch_list)
}

/// `ID` : `TYPE` => `expr` ;
fn gen_case_branch(iter: &mut BufferedTokenIter) -> Result<CaseBranch, String> {
  let Token::Ident { value, .. } = iter.get_required(&IDENT_TYPE)? else { unreachable!() };
  let id: Ident = Ident::from(value);

  iter.consume_required(&COLON_TYPE)?;

  let Token::Ident { value, .. } = iter.get_required(&IDENT_TYPE)? else { unreachable!() };
  let id_type: Type = Type::from(value);

  iter.consume_required(&CASE_BRANCH_TYPE)?;

  // each expression of case branch ends with semicolon
  let expr = gen_expression(iter, &SEMI_COLON_TYPE)?;
  iter.consume_required(&SEMI_COLON_TYPE)?;

  Ok(CaseBranch { id, id_type, expr })
}
