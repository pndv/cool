pub(super) mod cond_expr;
pub(super) mod loop_expr;
pub(super) mod case_expr;
pub(super) mod let_expr;
pub(super) mod dispatch_expr;

use crate::generators::expressions::dispatch_expr::gen_partial_dispatch_expr;
use crate::model::expressions::Expression;
use crate::model::Type;
use case_expr::gen_case_expression;
use cond_expr::gen_conditional_expression;
use dispatch_expr::gen_partial_cast_dispatch;
use let_expr::gen_let_expression;
use lexer::iter::token::{BaseTokenIter, BufferedTokenIter};
use lexer::model::constants::{ASSIGN_TYPE, CLOSE_CURL_TYPE, CLOSE_PAREN_TYPE, END_CASE_TYPE, END_IF_TYPE, END_LOOP_TYPE, IDENT_TYPE, NEW_TYPE, NOT_TYPE, OPEN_CURL_TYPE, OPEN_PAREN_TYPE, SEMI_COLON_TYPE, TILDE_TYPE};
use lexer::model::token::Token;
use loop_expr::gen_loop_expression;
use std::collections::VecDeque;
use std::mem::replace;

pub(super) fn gen_expression(iter: &mut BufferedTokenIter,
                             read_till_token: &Token) -> Result<Expression, String> {
  let mut expression_token_iter = iter.gen_iter_till(read_till_token);

  let partial_expressions = gen_partial_expressions(&mut expression_token_iter, read_till_token)?;
  let expr = reduce_expression_list(partial_expressions)?;
  Ok(expr)
}

fn gen_partial_expressions(iter: &mut BufferedTokenIter, read_till_token: &Token) -> Result<VecDeque<Expression>, String> {
  let mut expr_list: VecDeque<Expression> = VecDeque::new();
  while iter.has_next() {
    let Some(peek) = iter.peek() else { return Err(String::from("gen_partial_expressions: Unexpected EOF")) };
    match peek {
      Token::Empty | Token::Error { .. } | Token::Comment { .. } => {
        dbg!("get_expression_helper: Unexpected token {:?}", &peek);
        panic!("Unexpected token {:?}", peek);
      }

      Token::Ident { .. } => {
        let ident_token = iter.get_required(&IDENT_TYPE)?;

        let expr = if iter.peek_eq(&OPEN_PAREN_TYPE) {
          gen_partial_dispatch_expr(ident_token, iter)?
        } else {
          Expression::from(ident_token)
        };

        expr_list.push_back(expr);
      }

      Token::SelfType { .. } | Token::String { .. } | Token::Int { .. } | Token::True { .. } | Token::False { .. } => {
        let token = iter.next().unwrap_or_else(|| panic!("get_expression_helper: Error reading token"));
        expr_list.push_back(Expression::from(token));
      }

      Token::New { .. } => {
        iter.consume_required(&NEW_TYPE)?;

        let Token::Ident {value, ..} = iter.get_required(&IDENT_TYPE)? else { unreachable!() };
        expr_list.push_back(Expression::New { type_name: Type::from(value) });
      }

      Token::If { .. } => {
        let mut cond_expr_iter = iter.gen_iter_till(&END_IF_TYPE);
        let conditional_expr = gen_conditional_expression(&mut cond_expr_iter)?;
        expr_list.push_back(conditional_expr);
      }

      Token::While { .. } => {
        let mut loop_expr_iter = iter.gen_iter_till(&END_LOOP_TYPE);
        let loop_expr = gen_loop_expression(&mut loop_expr_iter)?;
        expr_list.push_back(loop_expr);
      }

      Token::Case { .. } => {
        let mut case_expr_iter = iter.gen_iter_till(&END_CASE_TYPE);
        let case_expr = gen_case_expression(&mut case_expr_iter)?;
        expr_list.push_back(case_expr);
      }

      Token::Let { .. } => {
        let let_expr = gen_let_expression(iter, read_till_token)?;
        expr_list.push_back(let_expr);
      }

      Token::OpenParen { .. } => {
        let mut single_expr_iter = iter.gen_iter_till(&CLOSE_PAREN_TYPE);
        let single_expr = gen_single_expr_within_paren(&mut single_expr_iter)?;
        expr_list.push_back(single_expr);
      }

      Token::OpenCurl { .. } => {
        let mut block_expr_iter = iter.gen_iter_till(&CLOSE_CURL_TYPE);
        let block_expr = gen_block_expr(&mut block_expr_iter)?;
        expr_list.push_back(block_expr);
      }

      Token::IsVoid { .. } | Token::Not { .. } | Token::Tilde { .. } => {
        let unary_expr = gen_unary_expr(iter, read_till_token)?;
        expr_list.push_back(unary_expr);
      }

      Token::Plus { .. } | Token::Minus { .. } | Token::Star { .. } | Token::ForwardSlash { .. } | Token::Less { .. } | Token::LessOrEqual { .. } | Token::Equal { .. } => {
        let partial_binary_expr = gen_partial_binary_expr(iter, read_till_token)?;
        expr_list.push_back(partial_binary_expr);
      }

      Token::Assign { .. } => {
        iter.consume_required(&ASSIGN_TYPE)?;
        let assign_expr = gen_expression(iter, read_till_token)?;
        expr_list.push_back(Expression::PartialAssign { expr: Box::new(assign_expr) });
      }

      Token::At { .. } | Token::Dot { .. } => {
        let partial_cast_dispatch = gen_partial_cast_dispatch(iter)?;
        expr_list.push_back(partial_cast_dispatch);
      }

      // Should never encounter these expressions, since no expression starts with these tokens
      Token::Then { .. } | Token::Else { .. } | Token::EndIf { .. } => panic!("Unexpected conditional branch {:?}", peek),
      Token::Loop { .. } | Token::EndLoop { .. } => panic!("Unexpected loop branch {:?}", peek),
      Token::CaseBranch { .. } | Token::Of { .. } | Token::EndCase { .. } => panic!("Unexpected case branch {:?}", peek),
      Token::In { .. } => panic!("Unexpected let branch {:?}", peek),

      _ => panic!("Unexpected token {:?}", peek),
    }
  }

  Ok(expr_list)
}

/// Collapse a list of expressions into a single expression; error otherwise
fn reduce_expression_list(mut expressions: VecDeque<Expression>) -> Result<Expression, String> {
  assert!(!expressions.is_empty(), "The expression list cannot be reduced on empty list");

  if expressions.len() == 1 {
    let e = expressions.pop_front().unwrap();
    if let Expression::PartialDispatch { .. } = e {
      return Ok(e.convert_to_dispatch());
    }

    assert!(!e.is_partial(), "List with a single partial expression");
    return Ok(e);
  }

  let reduce: Expression;

  let first = expressions.pop_front().unwrap();
  let second = expressions.get(0).unwrap().clone();

  if !second.is_partial() {
    let exps = expressions.clone().into_iter().collect::<Vec<_>>();
    for e in exps {
      println!("reduce_expression_list: {:?}", e)
    }
  }

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

  let _ = replace(&mut expressions[0], reduce);
  reduce_expression_list(expressions)
}

/// ...previously seen expression.. + {`+` | `-` | `*`| `/`| `<`| `<=`| `=`} expr
fn gen_partial_binary_expr(token_iter: &mut BufferedTokenIter, read_till_tokens: &Token) -> Result<Expression, String> {
  let binary_token = token_iter.next().unwrap_or_else(|| panic!("get_expression_helper: Error reading binary token"));
  let right = gen_expression(token_iter, read_till_tokens)?;
  let partial_binary_expr = Expression::PartialBinary { binary_token, right_expr: Box::new(right) };
  Ok(partial_binary_expr)
}

/// {`~` | `not` | `IsVoid`} expr
fn gen_unary_expr(iter: &mut BufferedTokenIter, read_till_tokens: &Token) -> Result<Expression, String> {
  // match / consume the unary token
  let unary_token = iter.next().unwrap_or_else(|| panic!("get_expression_helper: Error reading unary token"));

  let sub_expr = gen_expression(iter, read_till_tokens)?;

  let unary_expr = if unary_token == NOT_TYPE {
    Expression::Not { expr: Box::from(sub_expr) }
  } else if unary_token == TILDE_TYPE {
    Expression::Negate { expr: Box::from(sub_expr) }
  } else {
    Expression::IsVoid { expr: Box::from(sub_expr) }
  };

  Ok(unary_expr)
}

/// `(` expr `)`
fn gen_single_expr_within_paren(iter: &mut BufferedTokenIter) -> Result<Expression, String> {
  iter.consume_required(&OPEN_PAREN_TYPE)?;

  let expr = gen_expression(iter, &CLOSE_PAREN_TYPE)?;

  iter.consume_required(&CLOSE_PAREN_TYPE)?;

  Ok(expr)
}

/// `{` expr `;` {{ expr `;` ... }} `}`
fn gen_block_expr(iter: &mut BufferedTokenIter) -> Result<Expression, String> {
  iter.consume_required(&OPEN_CURL_TYPE)?;

  let mut block_expr_list = Vec::new();

  while iter.has_next() && !iter.peek_eq(&CLOSE_CURL_TYPE) { //Loop till end of block

    // each expression in block terminates with a semicolon
    let expr = gen_expression(iter, &SEMI_COLON_TYPE)?;
    iter.consume_required(&SEMI_COLON_TYPE)?;

    block_expr_list.push(expr);
  }

  assert!(!block_expr_list.is_empty(), "Block expression must contain at least one expression");
  iter.consume_required(&CLOSE_CURL_TYPE)?;

  Ok(Expression::Block { expr_list: block_expr_list })
}
