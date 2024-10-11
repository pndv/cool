use crate::expressions::gen_expression;
use crate::nodes::{Expression, Id, Type};
use crate::tokens::{match_required_token, peek_token_eq, FilteredTokensIterator, CASE_TYPE, COLON_TYPE, END_CASE_TYPE, IDENT_TYPE, LAMBDA_TYPE, OF_TYPE, SEMI_COLON_TYPE};

pub type CaseBranch = (Id, Type, Box<Expression>); // ID:TYPE => Expression 

pub(crate) fn gen_case_expression(token_iter: &mut FilteredTokensIterator) -> Expression {
  match_required_token(token_iter.next(), CASE_TYPE);

  let predicate_expr = gen_expression(token_iter, &OF_TYPE);
  match_required_token(token_iter.next(), OF_TYPE);

  let case_branches = gen_case_branch_list(token_iter);

  match_required_token(token_iter.next(), END_CASE_TYPE);

  Expression::Case {
    switch_expression: Box::from(predicate_expr),
    branches: case_branches,
  }
}

/// `ID` : `TYPE` => `expr` ; {{ `ID` : `TYPE` => `expr` ; }}
fn gen_case_branch_list(token_iter: &mut FilteredTokensIterator) -> Vec<CaseBranch> {
  let mut case_branch_list: Vec<CaseBranch> = Vec::new();

  while !peek_token_eq(token_iter, &END_CASE_TYPE) {
    case_branch_list.push(gen_case_branch(token_iter));
  }

  assert!(!case_branch_list.is_empty()); // case expression must have at least one branch

  case_branch_list
}

/// `ID` : `TYPE` => `expr` ;
fn gen_case_branch(token_iter: &mut FilteredTokensIterator) -> CaseBranch {
  let ident = match_required_token(token_iter.next(), IDENT_TYPE);
  let id: Id = Id::from(ident);

  match_required_token(token_iter.next(), COLON_TYPE);

  let type_ident = match_required_token(token_iter.next(), IDENT_TYPE);
  let type_id: Type = Type::from(type_ident);

  match_required_token(token_iter.next(), LAMBDA_TYPE);

  // each expression of case branch ends with semicolon
  let case_branch_expr = gen_expression(token_iter, &SEMI_COLON_TYPE);
  match_required_token(token_iter.next(), SEMI_COLON_TYPE);

  (id, type_id, Box::from(case_branch_expr))
}
