use crate::expressions;
use crate::nodes::Expression;
use crate::tokens::{match_required_token, FilteredTokensIterator, END_LOOP_TYPE, LOOP_TYPE, WHILE_TYPE};
use expressions::{gen_partial_expressions, reduce_expression_list};
use crate::terminal_tokens::{TERMINATE_TOKEN_END_LOOP, TERMINATE_TOKEN_LOOP};

pub(super) fn gen_loop_expression(token_iter: &mut FilteredTokensIterator) -> Expression {
  match_required_token(token_iter.next(), WHILE_TYPE);

  let predicate_expr_list = gen_partial_expressions(token_iter, &TERMINATE_TOKEN_LOOP);
  let predicate_expr = reduce_expression_list(predicate_expr_list);
  match_required_token(token_iter.next(), LOOP_TYPE);

  let loop_body_expr_list = gen_partial_expressions(token_iter, &TERMINATE_TOKEN_END_LOOP);
  let loop_body_expr = reduce_expression_list(loop_body_expr_list);
  match_required_token(token_iter.next(), END_LOOP_TYPE);

  Expression::Loop {
    predicate: Box::new(predicate_expr),
    body: Box::new(loop_body_expr),
  }
}