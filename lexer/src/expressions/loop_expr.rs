use crate::nodes::Expression;
use crate::tokens::{END_LOOP_TYPE, LOOP_TYPE, WHILE_TYPE};
use crate::{expressions, match_required_token, FilteredTokensIterator};
use expressions::{get_expression_helper, reduce_expression_list};
use std::collections::HashSet;

pub (super) fn gen_loop_expression(token_iter: &mut FilteredTokensIterator) -> Expression {
  match_required_token(token_iter.next(), WHILE_TYPE);

  let predicate_expr_list = get_expression_helper(token_iter, &HashSet::from([LOOP_TYPE]));
  let predicate_expr = reduce_expression_list(predicate_expr_list);
  match_required_token(token_iter.next(), LOOP_TYPE);

  let loop_body_expr_list = get_expression_helper(token_iter, &HashSet::from([END_LOOP_TYPE]));
  let loop_body_expr = reduce_expression_list(loop_body_expr_list);
  match_required_token(token_iter.next(), END_LOOP_TYPE);

  Expression::Loop {
    predicate: Box::new(predicate_expr),
    body: Box::new(loop_body_expr),
  }
}