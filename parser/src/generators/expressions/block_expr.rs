use crate::generators::expressions;
use crate::model::expressions::Expression;
use lexer::iter::token::{BaseTokenIter, BufferedTokenIter};
use lexer::model::constants::{CLOSE_CURL_TYPE, OPEN_CURL_TYPE, SEMI_COLON_TYPE};

/// `{` expr `;` {{ expr `;` ... }} `}`
pub(in crate::generators) fn gen_block_expr(iter: &mut BufferedTokenIter) -> Result<Expression, String> {
  iter.consume_required(&OPEN_CURL_TYPE)?;

  let mut block_expr_list = Vec::new();

  while iter.has_next() && !iter.peek_eq(&CLOSE_CURL_TYPE) { //Loop till end of block

    // each expression in block terminates with a semicolon
    let expr = expressions::gen_expression(iter, &SEMI_COLON_TYPE)?;
    iter.consume_required(&SEMI_COLON_TYPE)?;

    block_expr_list.push(expr);
  }

  assert!(!block_expr_list.is_empty(), "Block expression must contain at least one expression");
  iter.consume_required(&CLOSE_CURL_TYPE)?;

  Ok(Expression::Block { expr_list: block_expr_list })
}
