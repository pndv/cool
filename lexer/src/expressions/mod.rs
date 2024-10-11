﻿pub(super) mod cond_expr;
pub(super) mod loop_expr;
pub(super) mod case_expr;
pub(super) mod let_expr;
pub(super) mod dispatch_expr;

use crate::expressions::dispatch_expr::gen_partial_dispatch_expr;
use crate::nodes::{Expression, Type};
use crate::tokens::{generate_iter_till_token_or_end, match_required_token, peek_not_eq_or_eof, peek_token_eq, FilteredTokensIterator, Token, ASSIGN_TYPE, CLOSE_CURL_TYPE, CLOSE_PAREN_TYPE, IDENT_TYPE, NEW_TYPE, NOT_TYPE, OPEN_CURL_TYPE, OPEN_PAREN_TYPE, SEMI_COLON_TYPE, TILDE_TYPE};
use case_expr::gen_case_expression;
use dispatch_expr::gen_partial_cast_dispatch;
use let_expr::gen_let_expression;
use loop_expr::gen_loop_expression;
use std::collections::VecDeque;

pub(super) fn gen_expression(token_iter: &mut FilteredTokensIterator,
                             read_till_token: &Token) -> Expression {
  let mut expression_token_iter: FilteredTokensIterator = generate_iter_till_token_or_end(token_iter, read_till_token);

  /*  if cfg!(test) {
      for t in iter.clone() {
        println!("gen_expression: {:?}", t);
      }
    }
  */
  let partial_expressions = gen_partial_expressions(&mut expression_token_iter, &Token::EOF);
  let expr = reduce_expression_list(partial_expressions);
  expr
}

fn gen_partial_expressions(token_iter: &mut FilteredTokensIterator, read_till_token: &Token) -> VecDeque<Expression> {
  let mut expr_list: VecDeque<Expression> = VecDeque::new();
  while peek_not_eq_or_eof(token_iter, read_till_token) {
    let Some(peek) = token_iter.peek() else { panic!("get_expression_helper: Unexpected EOF") };
    match peek {
      Token::Empty |
      Token::Error { .. } |
      Token::Comment { .. } => {
        dbg!("get_expression_helper: Unexpected token {:?}", &peek);
        panic!("Unexpected token {:?}", peek);
      }

      Token::Ident { .. } => {
        let ident_token = match_required_token(token_iter.next(), IDENT_TYPE);

        let expr = if peek_token_eq(token_iter, &OPEN_PAREN_TYPE) {
          gen_partial_dispatch_expr(ident_token, token_iter)
        } else {
          Expression::from(ident_token)
        };

        expr_list.push_back(expr);
      }

      Token::SelfType { .. } |
      Token::String { .. } |
      Token::Int { .. } |
      Token::True { .. } |
      Token::False { .. } => {
        let token = token_iter.next().unwrap_or_else(|| panic!("get_expression_helper: Error reading token"));
        expr_list.push_back(Expression::from(token));
      }

      Token::New { .. } => {
        match_required_token(token_iter.next(), NEW_TYPE);

        let type_token = token_iter.next().unwrap_or_else(|| panic!("get_expression_helper: Error reading token"));
        expr_list.push_back(Expression::New { type_name: Type::from(type_token) });
      }

      Token::If { .. } => {
        let conditional_expr = cond_expr::gen_conditional_expression(token_iter);
        expr_list.push_back(conditional_expr);
      }

      Token::While { .. } => {
        let loop_expr = gen_loop_expression(token_iter);
        expr_list.push_back(loop_expr);
      }

      Token::Case { .. } => {
        let case_expr = gen_case_expression(token_iter);
        expr_list.push_back(case_expr);
      }

      Token::Let { .. } => {
        let let_expr = gen_let_expression(token_iter, read_till_token);
        expr_list.push_back(let_expr);
      }

      Token::OpenParen { .. } => {
        let single_expr = gen_single_expr_within_paren(token_iter);
        expr_list.push_back(single_expr);
      }

      Token::OpenCurl { .. } => {
        let block_expr = gen_block_expr(token_iter);
        expr_list.push_back(block_expr);
      }

      Token::IsVoid { .. } | Token::Not { .. } | Token::Tilde { .. } => {
        let unary_expr = gen_unary_expr(token_iter, read_till_token);
        expr_list.push_back(unary_expr);
      }

      Token::Plus { .. } | Token::Minus { .. } | Token::Star { .. } | Token::ForwardSlash { .. } |
      Token::Less { .. } | Token::LessOrEqual { .. } | Token::Equal { .. } => {
        let partial_binary_expr = gen_partial_binary_expr(token_iter, read_till_token);
        expr_list.push_back(partial_binary_expr);
      }

      Token::Assign { .. } => {
        match_required_token(token_iter.next(), ASSIGN_TYPE);
        let assign_expr = gen_expression(token_iter, read_till_token);
        expr_list.push_back(Expression::PartialAssign { expr: Box::new(assign_expr) });
      }

      Token::At { .. } | Token::Dot { .. } => {
        let partial_cast_dispatch = gen_partial_cast_dispatch(token_iter);
        expr_list.push_back(partial_cast_dispatch);
      }

      // Should never encounter these expressions, since no expression starts with these tokens
      Token::Then { .. } | Token::Else { .. } | Token::EndIf { .. } => panic!("Unexpected conditional branch {:?}", peek),
      Token::Loop { .. } | Token::EndLoop { .. } => panic!("Unexpected loop branch {:?}", peek),
      Token::Lambda { .. } | Token::Of { .. } | Token::EndCase { .. } => panic!("Unexpected case branch {:?}", peek),
      Token::In { .. } => panic!("Unexpected let branch {:?}", peek),

      _ => panic!("Unexpected token {:?}", peek),
    }
  }

  expr_list
}

/// Collapse a list of expressions into a single expression; error otherwise
fn reduce_expression_list(mut expressions: VecDeque<Expression>) -> Expression {
  assert!(!expressions.is_empty(), "The expression list cannot be reduced on empty list");

  if expressions.len() == 1 {
    let e = expressions.pop_front().unwrap();
    if let Expression::PartialDispatch { .. } = e {
      return e.convert_to_dispatch();
    }

    assert!(!e.is_partial(), "List with a single partial expression");
    return e;
  }

  let reduce: Expression;

  let first = expressions.pop_front().unwrap();
  let mut second = (expressions.get(0).unwrap()).clone();

  assert!(second.is_partial(), "Last expression must be partial");

  match second {
    Expression::PartialBinary { binary_token, right_expr } => {
      match binary_token {
        Token::Plus { .. } => reduce = Expression::Plus { left: Box::from(first), right: right_expr },
        Token::Minus { .. } => reduce = Expression::Minus { left: Box::from(first), right: right_expr },
        Token::Star { .. } => reduce = Expression::Multiply { left: Box::from(first), right: right_expr },
        Token::ForwardSlash { .. } => reduce = Expression::Divide { left: Box::from(first), right: right_expr },

        Token::Less { .. } => reduce = Expression::LessThan { left: Box::from(first), right: right_expr },
        Token::LessOrEqual { .. } => reduce = Expression::LessThanOrEqual { left: Box::from(first), right: right_expr },
        Token::Equal { .. } => reduce = Expression::Equal { left: Box::from(first), right: right_expr },

        _ => panic!("Unexpected token {:?}", binary_token),
      }
    }
    Expression::PartialAssign { expr } => {
      let Expression::Ident { name, .. } = first else { panic!("PartialAssign: join expression is not ident") };
      reduce = Expression::Assign { name, expr };
    }
    Expression::PartialCastDispatch { fn_name, cast_type, param_list } => {
      reduce = Expression::Dispatch { calling_expr: Box::from(first), cast_type, fn_name, param_list };
    }
    Expression::PartialDispatch { fn_name, param_list } => {
      reduce = Expression::Dispatch { calling_expr: Box::from(first), cast_type: None, fn_name, param_list };
    }
    _ => panic!("Incorrect expression {:?}", second)
  }

  let _ = std::mem::replace(&mut expressions[0], reduce);
  reduce_expression_list(expressions)
}

/// ...previously seen expression.. + {`+` | `-` | `*`| `/`| `<`| `<=`| `=`} expr
fn gen_partial_binary_expr(token_iter: &mut FilteredTokensIterator, read_till_tokens: &Token) -> Expression {
  let binary_token = token_iter.next().unwrap_or_else(|| panic!("get_expression_helper: Error reading binary token"));
  let right = gen_expression(token_iter, read_till_tokens);
  let partial_binary_expr = Expression::PartialBinary { binary_token, right_expr: Box::new(right) };
  partial_binary_expr
}

/// {`~` | `not` | `IsVoid`} expr
fn gen_unary_expr(token_iter: &mut FilteredTokensIterator, read_till_tokens: &Token) -> Expression {
  // match / consume the unary token
  let unary_token = token_iter.next().unwrap_or_else(|| panic!("get_expression_helper: Error reading unary token"));

  let sub_expr = gen_expression(token_iter, read_till_tokens);

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

  let expr = gen_expression(token_iter, &CLOSE_PAREN_TYPE);

  match_required_token(token_iter.next(), CLOSE_PAREN_TYPE);

  expr
}

/// `{` expr `;` {{ expr `;` ... }} `}`
fn gen_block_expr(token_iter: &mut FilteredTokensIterator) -> Expression {
  match_required_token(token_iter.next(), OPEN_CURL_TYPE);

  let mut block_expr_list = Vec::new();

  while !peek_token_eq(token_iter, &CLOSE_CURL_TYPE) { //Loop till end of block

    // each expression in block terminates with a semicolon
    let expr = gen_expression(token_iter, &SEMI_COLON_TYPE);
    match_required_token(token_iter.next(), SEMI_COLON_TYPE);

    block_expr_list.push(expr);
  }

  assert!(!block_expr_list.is_empty(), "Block expression must contain at least one expression");
  match_required_token(token_iter.next(), CLOSE_CURL_TYPE);

  Expression::Block { expr_list: block_expr_list }
}

#[cfg(test)]
mod test_expr {}
