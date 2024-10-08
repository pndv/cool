use std::collections::HashSet;
use expressions::{get_expression_helper, reduce_expression_list};
use crate::{expressions, peek_token_in, match_required_token, FilteredTokensIterator};
use crate::nodes::{Expression, Id, Type};
use crate::tokens::{Token, AT_TYPE, CLOSE_PAREN_TYPE, COMMA_TYPE, DOT_TYPE, IDENT_TYPE, OPEN_PAREN_TYPE};

/// ...expr (seen before)... { `@` TYPE } `.` ID `(` { expr {{ `,` expr }} } 
pub (super) fn gen_partial_cast_dispatch(token_iter: &mut FilteredTokensIterator) -> Expression {
  match_required_token(token_iter.next(), AT_TYPE);

  let type_ident = match_required_token(token_iter.next(), IDENT_TYPE);
  let cast_type: Type = Type::from(type_ident);

  match_required_token(token_iter.next(), DOT_TYPE);
  
  let fn_name_ident = match_required_token(token_iter.next(), IDENT_TYPE);
  let fn_name: Id = Id::from(fn_name_ident);

  match_required_token(token_iter.next(), OPEN_PAREN_TYPE);

  let param_list = gen_fn_params(token_iter);

  match_required_token(token_iter.next(), CLOSE_PAREN_TYPE);
  
  Expression::PartialCastDispatch {cast_type, fn_name, param_list }
}

pub (super) fn gen_partial_dispatch_expr(ident_token: Token, token_iter: &mut FilteredTokensIterator) -> Expression {
  match_required_token(token_iter.next(), OPEN_PAREN_TYPE);

  let param_list = gen_fn_params(token_iter);

  match_required_token(token_iter.next(), CLOSE_PAREN_TYPE);
  
  Expression::PartialDispatch {fn_name: Id::from(ident_token), param_list }
}

/// expr {{ `,` expr }} 
/// The `expr` can be followed by either `,` or `)`
/// 1. when next token is `,` => more parameters are present for the function call
/// 2. when next token is `)` => end of parameter list
fn gen_fn_params(token_iter: &mut FilteredTokensIterator) -> Vec<Expression> {
  match_required_token(token_iter.next(), OPEN_PAREN_TYPE);

  let terminal_tokens = HashSet::from([COMMA_TYPE, CLOSE_PAREN_TYPE]);
  let mut param_list: Vec<Expression> = Vec::new();

  while peek_token_in(token_iter, &terminal_tokens) {
    let param_expr_list = get_expression_helper(token_iter, &terminal_tokens);
    let param_expr = reduce_expression_list(param_expr_list);
    param_list.push(param_expr);
  }

  param_list
}