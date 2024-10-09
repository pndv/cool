﻿use crate::expressions::gen_expression;
use crate::nodes::{Expression, Formal, Id, Type};
use crate::terminal_tokens::{TERMINATE_TOKEN_FEATURE_METHOD_EXPR, TERMINATE_TOKEN_SEMI_COLON};
use crate::tokens::{match_required_token, peek_token_eq, peek_token_neq, peek_token_not_in, FilteredTokensIterator, Token, ASSIGN_TYPE, CLOSE_CURL_TYPE, CLOSE_PAREN_TYPE, COLON_TYPE, IDENT_TYPE, OPEN_CURL_TYPE, OPEN_PAREN_TYPE, SEMI_COLON_TYPE};

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Feature {
  feature_name: Id,
  formals: Option<Vec<Formal>>,
  return_type: Type,
  expr: Option<Box<Expression>>,
}

impl From<(Id, Option<Vec<Formal>>, Type, Box<Expression>)> for Feature {
  fn from((feature_name, formals, return_type, expr): (Id, Option<Vec<Formal>>, Type, Box<Expression>)) -> Self {
    Feature {
      feature_name,
      formals,
      return_type,
      expr: Some(expr),
    }
  }
}

impl From<(Id, Type, Box<Expression>)> for Feature {
  fn from((feature_name, return_type, expr): (Id, Type, Box<Expression>)) -> Self {
    Feature {
      feature_name,
      formals: None,
      return_type,
      expr: Some(expr),
    }
  }
}

impl From<(Id, Type)> for Feature {
  fn from((feature_name, return_type): (Id, Type)) -> Self {
    Feature {
      feature_name,
      formals: None,
      return_type,
      expr: None,
    }
  }
}

/// Features :-> {{ features; }}*
pub fn get_features(token_iter: &mut FilteredTokensIterator, read_till_tokens: &[Token]) -> Option<Vec<Feature>> {
  let mut features: Vec<Feature> = Vec::new();

  // `{` seen in calling method => read till closing `}` encountered for `class`
  while peek_token_not_in(token_iter, read_till_tokens) {
    let feature: Feature = get_feature(token_iter, &TERMINATE_TOKEN_SEMI_COLON);

    // Feature must terminate with a semicolon
    match_required_token(token_iter.next(), SEMI_COLON_TYPE);

    features.push(feature);
  }

  if features.is_empty() { None } else { Some(features) }
}

pub fn get_feature(token_iter: &mut FilteredTokensIterator, read_till_tokens: &[Token]) -> Feature {
  //Feature starts with ID
  let token = match_required_token(token_iter.next(), IDENT_TYPE);
  let ident_name: Id = Id::from(token);

  let feature: Feature = match token_iter.peek() {
    Some(peeked_token) if peeked_token.is_same_type(&COLON_TYPE) =>
      get_attribute_feature(ident_name, token_iter, read_till_tokens),

    Some(peeked_token) if peeked_token.is_same_type(&OPEN_PAREN_TYPE) =>
      get_method_feature(ident_name, token_iter, read_till_tokens),

    Some(t) => panic!("Incorrect token {:?}", t),

    None => panic!("Unexpected EOF"),
  };

  feature
}

fn get_method_feature(ident_name: Id, token_iter: &mut FilteredTokensIterator, read_till_tokens: &[Token]) -> Feature {
  match_required_token(token_iter.next(), OPEN_PAREN_TYPE);

  let mut formals: Option<Vec<Formal>> = None;

  // `(` seen in calling method => If the next token is not `)`, read formals list
  if peek_token_neq(token_iter, &CLOSE_PAREN_TYPE) {
    let vec_formals = crate::get_formals(token_iter);
    formals = Some(vec_formals);
  }

  match_required_token(token_iter.next(), CLOSE_PAREN_TYPE);

  match_required_token(token_iter.next(), COLON_TYPE);

  let token = match_required_token(token_iter.next(), IDENT_TYPE);
  let method_return_type = Type::from(token);

  match_required_token(token_iter.next(), OPEN_CURL_TYPE);

  let method_expr = gen_expression(token_iter, &TERMINATE_TOKEN_FEATURE_METHOD_EXPR);

  match_required_token(token_iter.next(), CLOSE_CURL_TYPE);

  (ident_name, formals, method_return_type, Box::from(method_expr)).into()
}

fn get_attribute_feature(ident_name: Id, token_iter: &mut FilteredTokensIterator, read_till_tokens: &[Token]) -> Feature {
  match_required_token(token_iter.next(), COLON_TYPE);

  let token = match_required_token(token_iter.next(), IDENT_TYPE);
  let method_return_type = Id::from(token);

  if peek_token_eq(token_iter, &ASSIGN_TYPE) {
    match_required_token(token_iter.next(), ASSIGN_TYPE);

    let method_expr = gen_expression(token_iter, read_till_tokens);
    (ident_name, method_return_type, Box::from(method_expr)).into()
  } else {
    (ident_name, method_return_type).into()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::tokens::get_filtered_token_iter;

  #[test]
  fn test_feature_method() {
    let file_path = "test_resources/features/feature.first_form";
    let mut token_iter: FilteredTokensIterator = get_filtered_token_iter(file_path);
    let feature: Feature = get_feature(&mut token_iter, &[]);
    let Feature { feature_name, formals, return_type, expr } = feature;

    let (name, ..) = feature_name;
    assert_eq!(name, "method2");

    assert!(formals.is_some());
    let Some(formal_list) = formals else { panic!("Formal list should not be empty") };
    assert_eq!(formal_list.len(), 2);

    /*    let Formal {formal_name, formal_type} = formal_list.pop().unwrap();
        assert_eq!(formal_name.0, "num2");
        assert_eq!(formal_type.0, "Int");
        let Formal {formal_name, formal_type} = formal_list.pop().unwrap();
        assert_eq!(formal_name.0, "num1");
        assert_eq!(formal_type.0, "Int");
    */
    let (formal_return_type, ..) = return_type;
    assert_eq!(formal_return_type, "B");

    assert!(expr.is_some());
    let Some(feature_expr) = expr else { panic!("feature expr should not be empty") };
    assert_eq!(feature_expr.get_type(), "Let");
  }

  #[test]
  fn test_feature_attribute_without_expr() {
    let file_path = "test_resources/features/feature.form_two_no_expr";
    let mut token_iter: FilteredTokensIterator = get_filtered_token_iter(file_path);
    let feature: Feature = get_feature(&mut token_iter, &[]);
    let Feature { feature_name, formals, return_type, expr } = feature;

    let (name, ..) = feature_name;
    assert_eq!(name, "population_map");

    let (formal_return_type, ..) = return_type;
    assert_eq!(formal_return_type, "String");

    assert!(formals.is_none());
    assert!(expr.is_none());
  }

  #[test]
  fn test_feature_list() {
    let file_path = "test_resources/features/feature.list";
    let mut token_iter: FilteredTokensIterator = get_filtered_token_iter(file_path);
    let features = get_features(&mut token_iter, &[]);
    assert!(features.is_some());

    let Some(feature_list) = features else { panic!("FeatureList should not be empty") };

    assert_eq!(feature_list.len(), 5);
  }

  #[test]
  fn test_feature_attribute_with_expr() {
    let file_path = "test_resources/features/feature.form_two_with_expr";
    let mut token_iter: FilteredTokensIterator = get_filtered_token_iter(file_path);
    let feature: Feature = get_feature(&mut token_iter, &[]);
    let Feature { feature_name, formals, return_type, expr } = feature;

    let (name, ..) = feature_name;
    assert_eq!(name, "vertices");

    let (formal_return_type, ..) = return_type;
    assert_eq!(formal_return_type, "VList");

    assert!(formals.is_none());

    assert!(expr.is_some());
    let Some(feature_expr) = expr else { panic!("feature expr should not be empty") };
    assert_eq!(feature_expr.get_type(), "New");
  }
}