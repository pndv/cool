use lexer::iter::token::TokenIter;
use crate::expressions::gen_expression;
use crate::expressions::Expression;
use lexer::tokens::{ END_LOOP_TYPE, LOOP_TYPE, WHILE_TYPE};

pub(super) fn gen_loop_expression(iter: &mut TokenIter) -> Expression {
  iter.consume_required(&WHILE_TYPE);

  let predicate_expr = gen_expression(iter, &LOOP_TYPE);
  iter.consume_required(&LOOP_TYPE);

  let loop_body_expr = gen_expression(iter, &END_LOOP_TYPE);
  iter.consume_required(&END_LOOP_TYPE);

  Expression::Loop {
    predicate: Box::new(predicate_expr),
    body: Box::new(loop_body_expr),
  }
}
