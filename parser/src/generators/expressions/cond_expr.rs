use crate::generators::expressions::gen_expression;
use crate::model::expressions::Expression;
use lex::iter::token::{BaseTokenIter, BufferedTokenIter};
use lex::model::constants::{ELSE_TYPE, END_IF_TYPE, IF_TYPE, THEN_TYPE};

pub(super) fn gen_conditional_expression(iter: &mut BufferedTokenIter) -> Result<Expression, String> {
  iter.consume_required(&IF_TYPE)?;
  let predicate_expr = gen_expression(iter, &THEN_TYPE)?;

  iter.consume_required(&THEN_TYPE)?;
  let then_expr = gen_expression(iter, &ELSE_TYPE)?;

  iter.consume_required(&ELSE_TYPE)?;
  let else_expr = gen_expression(iter, &END_IF_TYPE)?;

  iter.consume_required(&END_IF_TYPE)?;

  Ok(Expression::Conditional {
    predicate: Box::new(predicate_expr),
    then_expr: Box::new(then_expr),
    else_expr: Box::new(else_expr),
  })
}
