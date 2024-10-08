use crate::nodes::{Expression, Id, LetInit, Type};
use crate::tokens::{Token, ASSIGN_TYPE, COLON_TYPE, COMMA_TYPE, IDENT_TYPE, IN_TYPE, LET_TYPE};
use crate::{expressions, peek_token_eq, match_required_token, FilteredTokensIterator};
use std::collections::HashSet;
use expressions::get_expression_helper;

pub(crate) fn gen_let_expression(token_iter: &mut FilteredTokensIterator, read_till_tokens: &HashSet<Token>) -> Expression {
  match_required_token(token_iter.next(), LET_TYPE);

  let let_init_list = gen_let_init_list(token_iter);
  match_required_token(token_iter.next(), IN_TYPE);

  // Continue reading till calling code's end-token
  let let_in_expr_list = get_expression_helper(token_iter, read_till_tokens);
  let let_in_expr = expressions::reduce_expression_list(let_in_expr_list);

  Expression::Let {
    let_init: let_init_list,
    in_expr: Box::from(let_in_expr),
  }
}

/// ID : TYPE { <- expr } {{, ID : TYPE { <- expr } }} 
fn gen_let_init_list(token_iter: &mut FilteredTokensIterator) -> Vec<LetInit> {
  let mut init_list: Vec<LetInit> = Vec::new();

  loop {
    let init = gen_let_init(token_iter);
    init_list.push(init);

    if let Some(peek) = token_iter.peek() {

      // The initialisation list ends when `in` is encountered, or if any token other than `,` is seen 
      if peek.is_same_type(&IN_TYPE) || !peek.is_same_type(&COMMA_TYPE) {
        break;
      }
    } else {
      panic!("Token stream ended abruptly while generating `let` expression initialisation list")
    }

    // next token will be `,` =>  more initialisations remaining
    match_required_token(token_iter.next(), COMMA_TYPE);
  }

  assert!(!init_list.is_empty(), "Let expression initialisation list is empty");

  init_list
}

/// `Id` : `Type` {{ <- expr }}
fn gen_let_init(token_iter: &mut FilteredTokensIterator) -> LetInit {
  let ident = match_required_token(token_iter.next(), IDENT_TYPE);
  let id: Id = Id::from(ident);

  match_required_token(token_iter.next(), COLON_TYPE);

  let type_ident = match_required_token(token_iter.next(), IDENT_TYPE);
  let type_id: Type = Type::from(type_ident);

  let mut expr: Option<Box<Expression>> = None;

  if peek_token_eq(token_iter, &ASSIGN_TYPE) {
    match_required_token(token_iter.next(), ASSIGN_TYPE);

    // The end of this expression in `Let` is marked in two ways:
    // 1. `,` -> indicates the expression has ended, but more `LetInit` will follow
    // 2. `in` -> indicates the expression and the `LetInit` has ended
    let token_match_set = HashSet::from([COMMA_TYPE, IN_TYPE]);
    let intermediate_expr_list: Vec<Expression> = get_expression_helper(token_iter, &token_match_set);
    let init_expr = expressions::reduce_expression_list(intermediate_expr_list);

    expr = Some(Box::new(init_expr));
  }

  (id, type_id, expr)
}

mod test {
  use std::collections::HashSet;
  use crate::{convert_vec_filtered_iter, get_filtered_token_iter, FilteredTokensIterator};
  use crate::expressions::let_expr::gen_let_expression;
  use crate::tokens::{CASE_TYPE, CLOSE_CURL_TYPE, COMMA_TYPE, IDENT_TYPE, IN_TYPE, LET_TYPE};

  #[test]
  fn test_let_exp() {
    let file_path = "test_resources/expr.let";
    let mut token_iter: FilteredTokensIterator = get_filtered_token_iter(file_path);
    let read_till_tokens = HashSet::from([CLOSE_CURL_TYPE]);
    let expr = gen_let_expression(&mut token_iter, &read_till_tokens);
    println!("{:?}", expr);
    assert_eq!(expr.get_type(), "Let".to_string());
  }

}