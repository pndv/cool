use crate::expressions::gen_expression;
use crate::expressions::Expression;
use crate::tokens::{consume_required, FilteredTokensIterator, ELSE_TYPE, END_IF_TYPE, IF_TYPE, THEN_TYPE};

pub(super) fn gen_conditional_expression(token_iter: &mut FilteredTokensIterator) -> Expression {
  consume_required(token_iter, IF_TYPE);
  let predicate_expr = gen_expression(token_iter, &THEN_TYPE);

  consume_required(token_iter, THEN_TYPE);
  let then_expr = gen_expression(token_iter, &ELSE_TYPE);

  consume_required(token_iter, ELSE_TYPE);
  let else_expr = gen_expression(token_iter, &END_IF_TYPE);

  consume_required(token_iter, END_IF_TYPE);

  Expression::Conditional {
    predicate: Box::new(predicate_expr),
    then_expr: Box::new(then_expr),
    else_expr: Box::new(else_expr),
  }
}
