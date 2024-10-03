﻿use crate::nodes::{Class, Expression, Feature, Formal, Program, Type};
use crate::scanner::get_program_token_list;
use crate::scanner::tokens::Token;
use core::iter::Iterator;
use std::iter::{Filter, Peekable};
use std::vec::IntoIter;

pub mod scanner;
pub mod nodes;

type CommentFilter = fn(&Token) -> bool;
type FilteredTokensIterator = Peekable<Filter<IntoIter<Token>, CommentFilter>>;

pub fn analyse_lexical(file_path: &str) {
  let Ok(tokens) = get_program_token_list(file_path) else { panic!("Error reading file"); };

  if let Some(err) = check_tokens(&tokens) {
    panic!("{err}");
  }

  let is_not_comment: CommentFilter = is_not_comment;

  let mut token_iter: FilteredTokensIterator = tokens.into_iter()
                                                     .filter(is_not_comment)
                                                     .peekable();

  let mut program: Program = Program::new();

  while token_iter.peek().is_some() {
    let class = get_class(&mut token_iter);
    program.add_class(class);
  }
}

fn is_not_comment(token: &Token) -> bool {
  match token {
    Token::Comment { .. } => false,
    _ => true,
  }
}

fn get_class(token_iter: &mut FilteredTokensIterator) -> Class {
  // guaranteed to be non-empty at the start
  let mut token_option = token_iter.next();
  let mut token = match_required_token(token_option, Token::random_class());

  token_option = token_iter.next();
  token = match_required_token(token_option, Token::random_ident());
  let Token::Ident { value, .. } = token else { panic!("Unexpected token {:?}", token); };
  let type_name: Type = value;
  let mut parent_type: Option<Type> = None;

  token_option = token_iter.next();
  if token_option.is_some() && token_option.unwrap().is_same_type(&Token::random_inherits()) {
    token_option = token_iter.next();
    token = match_required_token(token_option, Token::random_ident());
    let Token::Ident { value, .. } = token else { panic!("Unexpected token {:?}", token) };
    let inherits_from: Type = value;
    parent_type = Some(inherits_from);
  }

  token_option = token_iter.next();
  token = match_required_token(token_option, Token::random_left_curl());

  let features = get_features(token_iter);

  token_option = token_iter.next();
  token = match_required_token(token_option, Token::random_right_curl());

  token_option = token_iter.next();
  token = match_required_token(token_option, Token::random_semi_colon());

  Class::new(type_name, parent_type, features)
}

fn get_features(token_iter: &mut FilteredTokensIterator) -> Option<Vec<Feature>> {
  let peek = token_iter.peek();
  if peek.is_none() || peek.unwrap() != &Token::random_semi_colon() {
    return None;
  }

  let mut features = Vec::new();
  let mut feature = get_feature(token_iter);
  features.push(feature);

  while token_iter.peek() == Some(&Token::random_semi_colon()) {
    match_required_token(token_iter.next(), Token::random_semi_colon()); // Consume ','

    feature = get_feature(token_iter);
    features.push(feature);
  }

  Some(features)
}

fn get_feature(token_iter: &mut FilteredTokensIterator) -> Feature {
  let mut token_option = token_iter.next();
  let mut token = match_required_token(token_option, Token::random_ident());

  let Token::Ident { value, .. } = token else { panic!("Unexpected token {:?}", token) };
  let ident_name = value;

  token_option = token_iter.next();
  token = match_required_token(token_option, Token::random_semi_colon());

  match token_iter.peek() {
    Some(peeked_token) if peeked_token.is_same_type(&Token::random_colon()) => 
      get_attribute_feature(ident_name, token_iter),
    
    Some(peeked_token) if peeked_token.is_same_type(&Token::random_left_paren()) => 
      get_method_feature(ident_name, token_iter),

    Some(t) =>  panic!("Incorrect token {:?}", t),
    
    None => panic!("Unexpected EOF"),
    
  }
}

fn get_method_feature(ident_name: String, token_iter: &mut FilteredTokensIterator) -> Feature {
  let mut token_option = token_iter.next();
  let mut token = match_required_token(token_option, Token::random_left_paren());

  let mut formals: Option<Vec<Formal>> = None;

  if token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&Token::random_right_paren()) {
    let vec_formals = get_formals(token_iter);
    formals = Some(vec_formals);
  }

  token_option = token_iter.next();
  token = match_required_token(token_option, Token::random_right_paren());

  token_option = token_iter.next();
  token = match_required_token(token_option, Token::random_colon());

  token_option = token_iter.next();
  token = match_required_token(token_option, Token::random_ident());
  let Token::Ident { value, .. } = token else { panic!("Unexpected token {:?}", token) };
  let method_return_type = value;

  token_option = token_iter.next();
  token = match_required_token(token_option, Token::random_left_curl());

  let method_expr = get_expression(token_iter);

  token_option = token_iter.next();
  token = match_required_token(token_option, Token::random_right_curl());

  Feature::method(ident_name, formals, method_return_type, Box::from(method_expr))
}

fn get_attribute_feature(ident_name: String, token_iter: &mut FilteredTokensIterator) -> Feature {
  let mut token_option = token_iter.next();
  let mut token = match_required_token(token_option, Token::random_colon());

  token_option = token_iter.next();
  token = match_required_token(token_option, Token::random_ident());
  let Token::Ident { value, .. } = token else { panic!("Unexpected token {:?}", token) };
  let method_return_type = value;

  if token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&Token::random_colon()) {
    token_option = token_iter.next();
    token = match_required_token(token_option, Token::random_assign_value());

    let method_expr = get_expression(token_iter);
    Feature::expr_attribute(ident_name, method_return_type, Box::from(method_expr))
  } else {
    Feature::simple_attribute(ident_name, method_return_type)
  }
}

fn get_formals(token_iter: &mut FilteredTokensIterator) -> Vec<Formal> {
  let mut formals = Vec::new();
  let mut formal = get_formal(token_iter);
  formals.push(formal);

  while token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&Token::random_comma()) {
    match_required_token(token_iter.next(), Token::random_comma()); // Consume ','

    formal = get_formal(token_iter);
    formals.push(formal);
  }

  formals
}

fn get_formal(token_iter: &mut FilteredTokensIterator) -> Formal {
  let mut token_option = token_iter.next();
  let mut token = match_required_token(token_option, Token::random_ident());

  let Token::Ident { value, .. } = token else { panic!("Unexpected token {:?}", token) };
  let ident_name = value;

  token_option = token_iter.next();
  let _ = match_required_token(token_option, Token::random_colon()); // consume colon

  token_option = token_iter.next();
  token = match_required_token(token_option, Token::random_ident());
  let Token::Ident { value, .. } = token else { panic!("Unexpected token {:?}", token) };
  let ident_type = value;

  Formal::new(ident_name, ident_type)
}

fn get_expression(token_iter: &mut FilteredTokensIterator) -> Expression {
  
}

fn match_required_token(token_option: Option<Token>, expected: Token) -> Token {
  if let Some(token) = token_option {
    if !token.is_same_type(&expected) {
      panic!("Unexpected token: {:?}", token);
    }

    token
  } else {
    panic!("Unexpected EOF");
  }
}

fn check_tokens(tokens: &Vec<Token>) -> Option<String> {
  let mut errors: String = String::from("");
  for token in tokens {
    match token {
      Token::Empty => {
        errors.push_str("Empty token! Parsing failed somewhere, can't specify details.\n");
      }
      Token::Error { error_char, line_num, line_pos } => {
        let x = format!("Error on line {line_num} at pos {line_pos}, offending character {error_char}.");
        errors.push_str(x.as_str())
      }
      _ => continue,
    }
  }

  if errors.is_empty() {
    None
  } else {
    Some(errors)
  }
}

