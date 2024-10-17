use crate::generators::expressions::gen_expression;
use crate::model::expressions::Expression;
use lexer::iter::token::{BaseTokenIter, BufferedTokenIter};
use lexer::model::constants::{END_LOOP_TYPE, LOOP_TYPE, WHILE_TYPE};

pub(super) fn gen_loop_expression(iter: &mut BufferedTokenIter) -> Result<Expression, String> {
  iter.consume_required(&WHILE_TYPE)?;

  let predicate_expr = gen_expression(iter, &LOOP_TYPE)?;
  iter.consume_required(&LOOP_TYPE)?;

  let loop_body_expr = gen_expression(iter, &END_LOOP_TYPE)?;
  iter.consume_required(&END_LOOP_TYPE)?;

  Ok(Expression::Loop {
    predicate: Box::new(predicate_expr),
    body: Box::new(loop_body_expr),
  })
}
