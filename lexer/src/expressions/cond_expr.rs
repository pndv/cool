use crate::expressions;
use crate::expressions::gen_expression;
use crate::nodes::Expression;
use crate::terminal_tokens::{TERMINATE_TOKEN_ELSE, TERMINATE_TOKEN_END_IF, TERMINATE_TOKEN_THEN};
use crate::tokens::{match_required_token, FilteredTokensIterator, ELSE_TYPE, END_IF_TYPE, IF_TYPE, THEN_TYPE};

pub(super) fn gen_conditional_expression(token_iter: &mut FilteredTokensIterator) -> Expression {
  match_required_token(token_iter.next(), IF_TYPE);
  let predicate_expr = gen_expression(token_iter, &TERMINATE_TOKEN_THEN);

  match_required_token(token_iter.next(), THEN_TYPE);
  let then_expr = gen_expression(token_iter, &TERMINATE_TOKEN_ELSE);

  match_required_token(token_iter.next(), ELSE_TYPE);
  let else_expr = gen_expression(token_iter, &TERMINATE_TOKEN_END_IF);

  match_required_token(token_iter.next(), END_IF_TYPE);

  Expression::Conditional {
    predicate: Box::new(predicate_expr),
    then_expr: Box::new(then_expr),
    else_expr: Box::new(else_expr),
  }
}

