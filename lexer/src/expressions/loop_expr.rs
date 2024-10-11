use crate::expressions::gen_expression;
use crate::expressions::Expression;
use crate::tokens::{consume_required, FilteredTokensIterator, END_LOOP_TYPE, LOOP_TYPE, WHILE_TYPE};

pub(super) fn gen_loop_expression(token_iter: &mut FilteredTokensIterator) -> Expression {
  consume_required(token_iter, WHILE_TYPE);

  let predicate_expr = gen_expression(token_iter, &LOOP_TYPE);
  consume_required(token_iter, LOOP_TYPE);

  let loop_body_expr = gen_expression(token_iter, &END_LOOP_TYPE);
  consume_required(token_iter, END_LOOP_TYPE);

  Expression::Loop {
    predicate: Box::new(predicate_expr),
    body: Box::new(loop_body_expr),
  }
}
