use crate::expressions::gen_expression;
use crate::nodes::Expression;
use crate::tokens::{match_required_token, FilteredTokensIterator, ELSE_TYPE, END_IF_TYPE, IF_TYPE, THEN_TYPE};

pub(super) fn gen_conditional_expression(token_iter: &mut FilteredTokensIterator) -> Expression {
  match_required_token(token_iter.next(), IF_TYPE);
  let predicate_expr = gen_expression(token_iter, &THEN_TYPE);

  match_required_token(token_iter.next(), THEN_TYPE);
  let then_expr = gen_expression(token_iter, &ELSE_TYPE);

  match_required_token(token_iter.next(), ELSE_TYPE);
  let else_expr = gen_expression(token_iter, &END_IF_TYPE);

  match_required_token(token_iter.next(), END_IF_TYPE);

  Expression::Conditional {
    predicate: Box::new(predicate_expr),
    then_expr: Box::new(then_expr),
    else_expr: Box::new(else_expr),
  }
}

