use crate::expressions::{gen_expression, Expression};
use crate::nodes::{Id, Type};
use crate::tokens::{consume_if_available, consume_required, gen_iter_till_token_or_end, match_required_token, peek_not_eq_or_eof, peek_token_eq, FilteredTokensIterator, Token, AT_TYPE, CLOSE_PAREN_TYPE, COMMA_TYPE, DOT_TYPE, IDENT_TYPE, OPEN_PAREN_TYPE};

/// ...expr (seen before)... { `@` TYPE } `.` ID `(` { expr {{ `,` expr }} } 
pub(super) fn gen_partial_cast_dispatch(token_iter: &mut FilteredTokensIterator) -> Expression {
  let mut cast_type: Option<Type> = None;

  if peek_token_eq(token_iter, &AT_TYPE) {
    consume_required(token_iter, AT_TYPE);

    let type_ident: Token = match_required_token(token_iter.next(), IDENT_TYPE);
    cast_type = Some(Type::from(type_ident));
  }

  consume_required(token_iter, DOT_TYPE);

  let fn_name_ident = match_required_token(token_iter.next(), IDENT_TYPE);
  let fn_name: Id = Id::from(fn_name_ident);

  let param_list = gen_fn_param_list(token_iter);

  Expression::PartialCastDispatch { cast_type, fn_name, param_list }
}

pub(super) fn gen_partial_dispatch_expr(ident_token: Token, token_iter: &mut FilteredTokensIterator) -> Expression {
  let param_list = gen_fn_param_list(token_iter);

  Expression::PartialDispatch { fn_name: Id::from(ident_token), param_list }
}

fn gen_fn_param_list(token_iter: &mut FilteredTokensIterator) -> Vec<Expression> {
  consume_required(token_iter, OPEN_PAREN_TYPE);
  let mut fn_param_gen_iter = gen_iter_till_token_or_end(token_iter, &CLOSE_PAREN_TYPE);
  consume_required(token_iter, CLOSE_PAREN_TYPE);

  let mut param_list: Vec<Expression> = Vec::new();

  /*  if cfg!(test) {
      let mut counter = 0;
      for t in fn_param_gen_iter.clone() {
        println!("gen_fn_param_list: >> {:?}", t);
        counter += 1;
      }
      println!("gen_fn_param_list: counter => {counter}");
    }
  */
  while peek_not_eq_or_eof(&mut fn_param_gen_iter, &COMMA_TYPE) {
    let param_expr = gen_expression(&mut fn_param_gen_iter, &COMMA_TYPE);
    param_list.push(param_expr);

    // consume ',' if we are not at the end of the stream
    consume_if_available(&mut fn_param_gen_iter, COMMA_TYPE);
  }

  param_list
}

