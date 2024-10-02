use core::iter::Iterator;
use std::iter::Peekable;
use std::vec::IntoIter;
use crate::nodes::{Class, Program};
use crate::scanner::get_program_token_list;
use crate::scanner::Token;

pub mod scanner;
pub mod nodes;

fn analyse_lexical(file_path: &str) {
  let Ok(tokens) = get_program_token_list(file_path);

  if let Some(err) = check_tokens(&tokens) {
    panic!("{err}");
  }

  let mut token_iter = tokens.into_iter().peekable();
  let mut program: Program = Program::new();

  while token_iter.peek().is_some() {
    let class = get_class(&mut token_iter);
    program.add_class(class);
  }
}

fn get_class(token_iter: &mut Peekable<IntoIter<Token>>) -> Class {
  // guaranteed to be non-empty at the start
  let mut token_option = token_iter.next();
  let mut token = match_required_token(token_option, Token::Class);

  match token_iter.next() {
    Some(Class) => {},
    Some(token) => panic!("Unexpected {token:?}"),
    None => panic!("Unexpected EOF"),
  }

  match token_iter.next() {
    None => {}
    Some(Token::Ident {}) => {}
  }
}

fn match_required_token(token_option: Option<Token>, expected: Token) -> Token {
  if token_option == None {
    panic!("Unexpected EOF");
  }

  let Some(token) = token_option;
  if token != expected {
    panic!("Unexpected token: {:?}", token);
  }

  token
}

fn match_optional_token(token_option: Option<Token>, expected: Token) -> Token {
  if token_option == None {
    panic!("Unexpected EOF");
  }

  let Some(token) = token_option;
  if token != expected {
    panic!("Unexpected token: {:?}", token);
  }

  token
}

fn check_tokens(tokens: &Vec<Token>) -> Option<String> {
  let mut errors: String = String::from("");
  for token in tokens {
    match token {
      Token::Empty  => {
        errors.push_str( "Empty token! Parsing failed somewhere, can't specify details.\n");
      }
      Token::Error { error_char, line_num, line_pos } => {
        let x = format!("Error on line {line_num} at pos {line_pos}, offending character {error_char}.");
        errors.push_str(x.as_str())
      }
      _ => continue,
    }
  }

  if errors.is_empty(){
    None
  } else {
    Some(errors)
  }
}

