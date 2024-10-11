use crate::expressions::gen_expression;
use crate::nodes::Expression;
use crate::tokens::{match_required_token, FilteredTokensIterator, END_LOOP_TYPE, LOOP_TYPE, WHILE_TYPE};

pub(super) fn gen_loop_expression(token_iter: &mut FilteredTokensIterator) -> Expression {
  match_required_token(token_iter.next(), WHILE_TYPE);

  let predicate_expr = gen_expression(token_iter, &LOOP_TYPE);
  match_required_token(token_iter.next(), LOOP_TYPE);

  let loop_body_expr = gen_expression(token_iter, &END_LOOP_TYPE);
  match_required_token(token_iter.next(), END_LOOP_TYPE);

  Expression::Loop {
    predicate: Box::new(predicate_expr),
    body: Box::new(loop_body_expr),
  }
}