use crate::nodes::Expression;
use crate::tokens::{ELSE_TYPE, END_IF_TYPE, IF_TYPE, THEN_TYPE};
use crate::{expressions, match_required_token, FilteredTokensIterator};
use std::clone::Clone;
use std::collections::HashSet;

pub (super) fn gen_conditional_expression(token_iter: &mut FilteredTokensIterator) -> Expression {
  match_required_token(token_iter.next(), IF_TYPE);
  let predicate_expr_list = expressions::get_expression_helper(token_iter, &HashSet::from([THEN_TYPE]));
  let predicate_expr = expressions::reduce_expression_list(predicate_expr_list);

  match_required_token(token_iter.next(), THEN_TYPE);
  let then_expr_list = expressions::get_expression_helper(token_iter, &HashSet::from([ELSE_TYPE]));
  let then_expr = expressions::reduce_expression_list(then_expr_list);

  match_required_token(token_iter.next(), ELSE_TYPE);
  let else_expr_list = expressions::get_expression_helper(token_iter, &HashSet::from([END_IF_TYPE]));
  let else_expr = expressions::reduce_expression_list(else_expr_list);

  match_required_token(token_iter.next(), END_IF_TYPE);

  Expression::Conditional {
    predicate: Box::new(predicate_expr),
    then_expr: Box::new(then_expr),
    else_expr: Box::new(else_expr),
  }
}