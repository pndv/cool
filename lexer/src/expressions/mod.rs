pub(super) mod cond_expr;
pub(super) mod loop_expr;
pub(super) mod case_expr;
pub(super) mod let_expr;
pub(super) mod dispatch_expr;

use crate::nodes::{Expression, Type};
use crate::tokens::{Token, ASSIGN_TYPE, CLOSE_CURL_TYPE, CLOSE_PAREN_TYPE, NEW_TYPE, NOT_TYPE, OPEN_CURL_TYPE, OPEN_PAREN_TYPE, SEMI_COLON_TYPE, TILDE_TYPE};
use crate::{match_peeked_token, match_peeked_token_in_list, match_required_token, FilteredTokensIterator};
use case_expr::gen_case_expression;
use let_expr::gen_let_expression;
use loop_expr::gen_loop_expression;
use std::collections::HashSet;
use dispatch_expr::gen_partial_cast_dispatch;

fn get_expression_helper(token_iter: &mut FilteredTokensIterator, read_till_tokens: &HashSet<Token>) -> Vec<Expression> {
  let mut expr_list: Vec<Expression> = Vec::new();
  while !match_peeked_token_in_list(token_iter, read_till_tokens) {
    let Some(peek) = token_iter.peek() else { panic!("get_expression_helper: Unexpected EOF") };
    match peek {
      Token::Empty |
      Token::Error { .. } |
      Token::Comment { .. } => {
        dbg!("get_expression_helper: Unexpected token {:?}", &peek);
        panic!("Unexpected token {:?}", peek);
      }

      Token::SelfType { .. } |
      Token::Ident { .. } |
      Token::String { .. } |
      Token::Int { .. } |
      Token::True { .. } |
      Token::False { .. } => {
        let token = token_iter.next().unwrap_or_else(|| panic!("get_expression_helper: Error reading token"));
        expr_list.push(Expression::from(token));
      }

      Token::New { .. } => {
        match_required_token(token_iter.next(), NEW_TYPE);

        let type_token = token_iter.next().unwrap_or_else(|| panic!("get_expression_helper: Error reading token"));
        expr_list.push(Expression::New { type_name: Type::from(type_token) });
      }

      Token::If { .. } => {
        let conditional_expr = cond_expr::gen_conditional_expression(token_iter);
        expr_list.push(conditional_expr);
      }

      Token::While { .. } => {
        let loop_expr = gen_loop_expression(token_iter);
        expr_list.push(loop_expr);
      }

      Token::Case { .. } => {
        let case_expr = gen_case_expression(token_iter);
        expr_list.push(case_expr);
      }

      Token::Let { .. } => {
        let let_expr = gen_let_expression(token_iter, read_till_tokens);
        expr_list.push(let_expr);
      }

      Token::OpenParen { .. } => {
        let single_expr = gen_single_expr_within_paren(token_iter);
        expr_list.push(single_expr);
      }

      Token::OpenCurl { .. } => {
        let block_expr = gen_block_expr(token_iter);
        expr_list.push(block_expr);
      }

      Token::IsVoid { .. } | Token::Not { .. } | Token::Tilde { .. } => {
        let unary_expr = gen_unary_expr(token_iter, read_till_tokens);
        expr_list.push(unary_expr);
      }

      Token::Plus { .. } | Token::Minus { .. } | Token::Star { .. } | Token::ForwardSlash { .. } |
      Token::Less { .. } | Token::LessOrEqual { .. } | Token::Equal { .. } => {
        let partial_binary_expr = gen_partial_binary_expr(token_iter, read_till_tokens);
        expr_list.push(partial_binary_expr);
      }

      Token::Assign { .. } => {
        match_required_token(token_iter.next(), ASSIGN_TYPE);
        let partial_assign_expr_list = get_expression_helper(token_iter, read_till_tokens);
        let assign_expr = reduce_expression_list(partial_assign_expr_list);
        expr_list.push(Expression::PartialAssign { expr: Box::new(assign_expr) });
      }

      Token::At { .. } => {
        let partial_cast_dispatch = gen_partial_cast_dispatch(token_iter);
        expr_list.push(partial_cast_dispatch);
      }
      
      // Should never encounter these expressions, since no expression starts with these tokens
      Token::Then { .. } | Token::Else { .. } | Token::EndIf { .. } => panic!("Unexpected conditional branch {:?}", peek),
      Token::Loop { .. } | Token::EndLoop { .. } => panic!("Unexpected loop branch {:?}", peek),
      Token::Lambda { .. } | Token::Of { .. } | Token::EndCase { .. } => panic!("Unexpected case branch {:?}", peek),
      Token::In { .. } => panic!("Unexpected let branch {:?}", peek),
      Token::Dot { .. } => panic!("Unexpected dispatch token {:?}", peek),
      _ => panic!("Unexpected token {:?}", peek),
    }
  }

  expr_list
}

/// Collapse a list of expressions into a single expression; error otherwise
fn reduce_expression_list(expressions: Vec<Expression>) -> Expression {
  todo!()
}

/// ...previously seen expression.. + {`+` | `-` | `*`| `/`| `<`| `<=`| `=`} expr
fn gen_partial_binary_expr(token_iter: &mut FilteredTokensIterator, read_till_tokens: &HashSet<Token>) -> Expression {
  let binary_token = token_iter.next().unwrap_or_else(|| panic!("get_expression_helper: Error reading binary token"));
  let right_expr_list = get_expression_helper(token_iter, read_till_tokens);
  let right = reduce_expression_list(right_expr_list);
  let partial_binary_expr = Expression::PartialBinary { binary_token, right_expr: Box::new(right) };
  partial_binary_expr
}

/// {`~` | `not` | `IsVoid`} expr
fn gen_unary_expr(token_iter: &mut FilteredTokensIterator, read_till_tokens: &HashSet<Token>) -> Expression {
  // match / consume the unary token
  let unary_token = token_iter.next().unwrap_or_else(|| panic!("get_expression_helper: Error reading unary token"));

  let sub_expr_list = get_expression_helper(token_iter, read_till_tokens);
  let sub_expr = reduce_expression_list(sub_expr_list);

  let unary_expr = if unary_token.is_same_type(&NOT_TYPE) {
    Expression::Not { expr: Box::from(sub_expr) }
  } else if unary_token.is_same_type(&TILDE_TYPE) {
    Expression::Negate { expr: Box::from(sub_expr) }
  } else {
    Expression::IsVoid { expr: Box::from(sub_expr) }
  };

  unary_expr
}

/// `(` expr `)`
fn gen_single_expr_within_paren(token_iter: &mut FilteredTokensIterator) -> Expression {
  match_required_token(token_iter.next(), OPEN_PAREN_TYPE);

  let expr_list = get_expression_helper(token_iter, &HashSet::from([CLOSE_PAREN_TYPE]));
  let expr = reduce_expression_list(expr_list);

  match_required_token(token_iter.next(), CLOSE_PAREN_TYPE);

  expr
}

/// `{` expr `;` {{ expr `;` ... }} `}`
fn gen_block_expr(token_iter: &mut FilteredTokensIterator) -> Expression {
  match_required_token(token_iter.next(), OPEN_CURL_TYPE);

  let mut block_expr_list = Vec::new();

  while !match_peeked_token(token_iter, &CLOSE_CURL_TYPE) { //Loop till end of block

    // each expression in block terminates with a semicolon
    let expr_list = get_expression_helper(token_iter, &HashSet::from([SEMI_COLON_TYPE]));
    let expr = reduce_expression_list(expr_list);

    block_expr_list.push(expr);
  }

  assert!(!block_expr_list.is_empty(), "Block expression must contain at least one expression");
  match_required_token(token_iter.next(), CLOSE_CURL_TYPE);

  Expression::Block { expr_list: block_expr_list }
}

#[cfg(test)]
mod test_expr {
  
}