use crate::expressions::{gen_expression, Expression, LetInit};
use crate::nodes::{Id, Type};
use crate::tokens::{consume_if_available, consume_required, gen_iter_till_token_or_end, match_required_token, peek_not_eq_or_eof, peek_token_eq, FilteredTokensIterator, Token, ASSIGN_TYPE, COLON_TYPE, COMMA_TYPE, IDENT_TYPE, IN_TYPE, LET_TYPE};

pub(crate) fn gen_let_expression(token_iter: &mut FilteredTokensIterator, read_till_token: &Token) -> Expression {
  consume_required(token_iter, LET_TYPE);

  let mut init_list_iter = gen_iter_till_token_or_end(token_iter, &IN_TYPE);

  /*  if cfg!(test) {
      let iter = init_list_iter.clone().collect::<Vec<_>>();
      println!("init_list_iter size {}", iter.len());
      for t in iter {
        println!("gen_let_expression: init_list_iter: {:?}", t)
      }
    }
  */  let let_init_list = gen_let_init_list(&mut init_list_iter);

  /*  if cfg!(test) {
      let iter = token_iter.clone().collect::<Vec<_>>();
      println!("token_iter size {}", iter.len());
      for t in iter {
        println!("gen_let_expression: token_iter: {:?}", t)
      }
    }
  */
  consume_required(token_iter, IN_TYPE);

  // Continue reading till calling code's end-token
  let let_in_expr = gen_expression(token_iter, read_till_token);

  Expression::Let {
    let_init: let_init_list,
    in_expr: Box::from(let_in_expr),
  }
}

/// ID : TYPE { <- expr } {{, ID : TYPE { <- expr } }} 
fn gen_let_init_list(token_iter: &mut FilteredTokensIterator) -> Vec<LetInit> {
  let mut init_list: Vec<LetInit> = Vec::new();

  /*  if cfg!(test) {
      let tokens = token_iter.clone().collect::<Vec<_>>();
      println!("gen_let_init_list: SIZE: {:?}", tokens.len());
      for t in tokens {
        println!("gen_let_init_list: {:?}", t);
      }
    }
  */
  while peek_not_eq_or_eof(token_iter, &COMMA_TYPE) {
    let init = gen_let_init(token_iter);
    init_list.push(init);

    consume_if_available(token_iter, COMMA_TYPE);
  }

  assert!(!init_list.is_empty(), "Let expression initialisation list is empty");

  init_list
}

/// `Id` : `Type` {{ <- expr }}
fn gen_let_init(token_iter: &mut FilteredTokensIterator) -> LetInit {
  /*
    if cfg!(test) {
      let tokens = token_iter.clone().collect::<Vec<_>>();
      println!("gen_let_init: SIZE: {:?}", tokens.len());
      for t in tokens {
        println!("gen_let_init: {:?}", t);
      }
    }
  */
  let ident = match_required_token(token_iter.next(), IDENT_TYPE);
  let id: Id = Id::from(ident);

  consume_required(token_iter, COLON_TYPE);

  let type_ident = match_required_token(token_iter.next(), IDENT_TYPE);
  let type_id: Type = Type::from(type_ident);

  let mut expr: Option<Box<Expression>> = None;

  if peek_token_eq(token_iter, &ASSIGN_TYPE) {
    consume_required(token_iter, ASSIGN_TYPE);

    // read till either encounter ',' or end of iter
    let init_expr = gen_expression(token_iter, &COMMA_TYPE);

    expr = Some(Box::new(init_expr));
  }

  (id, type_id, expr)
}

mod test {
  use crate::expressions::let_expr::gen_let_expression;
  use crate::tokens::{get_filtered_token_iter, FilteredTokensIterator, Token};

  #[test]
  fn test_let_exp() {
    let file_path = "test_resources/expressions/expr.let";
    let mut token_iter: FilteredTokensIterator = get_filtered_token_iter(file_path);
    let expr = gen_let_expression(&mut token_iter, &Token::EOF);
    println!("{:?}", expr);
    assert_eq!(expr.get_type(), "Let".to_string());
  }
}