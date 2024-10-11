use crate::scanner::get_file_token_list;
use mem::discriminant;
use std::iter::{Filter, Peekable};
use std::mem;
use std::vec::IntoIter;

pub const DOT: char = '.';
pub const AT: char = '@';
pub const TILDE: char = '~';
pub const STAR: char = '*';
pub const FORWARD_SLASH: char = '/';
pub const PLUS: char = '+';
pub const MINUS: char = '-';
pub const LESS_THAN: char = '<';
pub const GREATER_THAN: char = '>';
pub const EQUAL: char = '=';
pub const DOUBLE_QUOTE: char = '"';
pub const SEMI_COLON: char = ';';
pub const COLON: char = ':';
pub const COMMA: char = ',';
pub const OPEN_PAREN: char = '(';
pub const CLOSE_PAREN: char = ')';
pub const OPEN_CURL: char = '{';
pub const CLOSE_CURL: char = '}';

const KEYWORD_CLASS: &str = "class";
const KEYWORD_INHERITS: &str = "inherits";
const KEYWORD_COND_IF_START: &str = "if";
const KEYWORD_COND_THEN: &str = "then";
const KEYWORD_COND_ELSE: &str = "else";
const KEYWORD_COND_IF_END: &str = "fi";
const KEYWORD_IN: &str = "in";
const KEYWORD_LET: &str = "let";
const KEYWORD_IS_VOID: &str = "isvoid";
const KEYWORD_NOT: &str = "not";
const KEYWORD_LOOP: &str = "loop";
const KEYWORD_LOOP_END: &str = "pool";
const KEYWORD_WHILE: &str = "while";
const KEYWORD_CASE_START: &str = "case";
const KEYWORD_CASE_END: &str = "esac";
const KEYWORD_NEW: &str = "new";
const KEYWORD_OF: &str = "of";
const KEYWORD_FALSE: &str = "false";
const KEYWORD_TRUE: &str = "true";
const KEYWORD_SELF_TYPE: &str = "SELF_TYPE";

pub(crate) const IDENT_TYPE: Token = Token::Ident { value: String::new(), line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const LAMBDA_TYPE: Token = Token::Lambda { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const DOT_TYPE: Token = Token::Dot { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const COMMA_TYPE: Token = Token::Comma { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const ASSIGN_TYPE: Token = Token::Assign { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const COLON_TYPE: Token = Token::Colon { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const SEMI_COLON_TYPE: Token = Token::SemiColon { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const OPEN_PAREN_TYPE: Token = Token::OpenParen { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const CLOSE_PAREN_TYPE: Token = Token::CloseParen { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const OPEN_CURL_TYPE: Token = Token::OpenCurl { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const CLOSE_CURL_TYPE: Token = Token::CloseCurl { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const CLASS_TYPE: Token = Token::Class { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const INHERITS_TYPE: Token = Token::Inherits { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const IF_TYPE: Token = Token::If { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const THEN_TYPE: Token = Token::Then { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const ELSE_TYPE: Token = Token::Else { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const END_IF_TYPE: Token = Token::EndIf { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const LOOP_TYPE: Token = Token::Loop { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const END_LOOP_TYPE: Token = Token::EndLoop { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const IN_TYPE: Token = Token::In { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const OF_TYPE: Token = Token::Of { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const END_CASE_TYPE: Token = Token::EndCase { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const WHILE_TYPE: Token = Token::While { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const LET_TYPE: Token = Token::Let { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const CASE_TYPE: Token = Token::Case { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const NEW_TYPE: Token = Token::New { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const NOT_TYPE: Token = Token::Not { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const TILDE_TYPE: Token = Token::Tilde { line_num: u32::MAX, line_pos: u32::MAX };
pub(crate) const AT_TYPE: Token = Token::At { line_num: u32::MAX, line_pos: u32::MAX };

type CommentFilter = fn(&Token) -> bool;
pub(crate) type FilteredTokensIterator = Peekable<Filter<IntoIter<Token>, CommentFilter>>;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub(crate) enum Token {
  Empty,
  EOF, //end of file
  Error { error_char: String, line_num: u32, line_pos: u32 },
  Comment { value: String, line_num: u32, line_pos: u32 },

  Ident { value: String, line_num: u32, line_pos: u32 },

  Dot { line_num: u32, line_pos: u32 },
  Comma { line_num: u32, line_pos: u32 },

  Assign { line_num: u32, line_pos: u32 }, // `<-`
  Lambda { line_num: u32, line_pos: u32 }, // `=>`

  At { line_num: u32, line_pos: u32 },
  Tilde { line_num: u32, line_pos: u32 },

  Plus { line_num: u32, line_pos: u32 },
  Minus { line_num: u32, line_pos: u32 },
  Star { line_num: u32, line_pos: u32 },
  ForwardSlash { line_num: u32, line_pos: u32 },

  LessOrEqual { line_num: u32, line_pos: u32 },
  Less { line_num: u32, line_pos: u32 },
  Equal { line_num: u32, line_pos: u32 },

  Colon { line_num: u32, line_pos: u32 },
  SemiColon { line_num: u32, line_pos: u32 },

  OpenParen { line_num: u32, line_pos: u32 },
  CloseParen { line_num: u32, line_pos: u32 },
  OpenCurl { line_num: u32, line_pos: u32 },
  CloseCurl { line_num: u32, line_pos: u32 },

  Class { line_num: u32, line_pos: u32 },
  Inherits { line_num: u32, line_pos: u32 },

  If { line_num: u32, line_pos: u32 },
  Then { line_num: u32, line_pos: u32 },
  Else { line_num: u32, line_pos: u32 },
  EndIf { line_num: u32, line_pos: u32 },

  While { line_num: u32, line_pos: u32 },
  Loop { line_num: u32, line_pos: u32 },
  EndLoop { line_num: u32, line_pos: u32 },

  Let { line_num: u32, line_pos: u32 },
  In { line_num: u32, line_pos: u32 },

  Case { line_num: u32, line_pos: u32 },
  Of { line_num: u32, line_pos: u32 },
  EndCase { line_num: u32, line_pos: u32 },

  New { line_num: u32, line_pos: u32 },
  IsVoid { line_num: u32, line_pos: u32 },
  Not { line_num: u32, line_pos: u32 },

  Int { value: i32, line_num: u32, line_pos: u32 },
  String { value: String, line_num: u32, line_pos: u32 },
  True { line_num: u32, line_pos: u32 },
  False { line_num: u32, line_pos: u32 },

  SelfType { line_num: u32, line_pos: u32 },
}

impl Token {
  pub(crate) fn get_keyword(&self) -> Option<Token> {
    match self {
      Token::Ident { ref value, ref line_num, ref line_pos } => {
        let lower_case = value.to_lowercase();
        let v = lower_case.as_str();

        match v {
          KEYWORD_CLASS => Some(Token::Class { line_num: *line_num, line_pos: *line_pos }),
          KEYWORD_INHERITS => Some(Token::Inherits { line_num: *line_num, line_pos: *line_pos }),

          KEYWORD_COND_IF_START => Some(Token::If { line_num: *line_num, line_pos: *line_pos }),
          KEYWORD_COND_THEN => Some(Token::Then { line_num: *line_num, line_pos: *line_pos }),
          KEYWORD_COND_ELSE => Some(Token::Else { line_num: *line_num, line_pos: *line_pos }),
          KEYWORD_COND_IF_END => Some(Token::EndIf { line_num: *line_num, line_pos: *line_pos }),

          KEYWORD_IN => Some(Token::In { line_num: *line_num, line_pos: *line_pos }),
          KEYWORD_LET => Some(Token::Let { line_num: *line_num, line_pos: *line_pos }),

          KEYWORD_IS_VOID => Some(Token::IsVoid { line_num: *line_num, line_pos: *line_pos }),
          KEYWORD_NOT => Some(Token::Not { line_num: *line_num, line_pos: *line_pos }),

          KEYWORD_LOOP => Some(Token::Loop { line_num: *line_num, line_pos: *line_pos }),
          KEYWORD_LOOP_END => Some(Token::EndLoop { line_num: *line_num, line_pos: *line_pos }),
          KEYWORD_WHILE => Some(Token::While { line_num: *line_num, line_pos: *line_pos }),

          KEYWORD_CASE_START => Some(Token::Case { line_num: *line_num, line_pos: *line_pos }),
          KEYWORD_OF => Some(Token::Of { line_num: *line_num, line_pos: *line_pos }),
          KEYWORD_CASE_END => Some(Token::EndCase { line_num: *line_num, line_pos: *line_pos }),

          KEYWORD_NEW => Some(Token::New { line_num: *line_num, line_pos: *line_pos }),
          KEYWORD_SELF_TYPE => Some(Token::SelfType { line_num: *line_num, line_pos: *line_pos }),
          KEYWORD_FALSE if value.starts_with('f') => Some(Token::False { line_num: *line_num, line_pos: *line_pos }),
          KEYWORD_TRUE if value.starts_with('t') => Some(Token::True { line_num: *line_num, line_pos: *line_pos }),

          &_ => None,
        }
      }
      _ => None,
    }
  }

  pub(crate) fn is_same_type(&self, other: &Token) -> bool {
    discriminant(self) == discriminant(other)
  }
}

#[derive(PartialEq, Debug)]
pub(crate) enum WhiteSpace {
  Space,
  Tab,
  NewLine,
  CarriageReturn,
  FormFeed,
  VerticalTab,
}

impl WhiteSpace {
  pub(crate) fn get(value: char) -> WhiteSpace {
    match value {
      ' ' => WhiteSpace::Space,
      '\t' => WhiteSpace::Tab,
      '\n' => WhiteSpace::NewLine,
      '\r' => WhiteSpace::CarriageReturn,
      '\u{c}' => WhiteSpace::FormFeed,
      '\u{b}' => WhiteSpace::VerticalTab,

      _ => panic!("Not a whitespace {value}")
    }
  }

  pub(crate) fn is_whitespace(value: char) -> bool {
    value == ' ' ||
        value == '\t' ||
        value == '\n' ||
        value == '\r' ||
        value == '\u{c}' ||
        value == '\u{b}'
  }
}

pub(crate) fn get_filtered_token_iter(file_path: &str) -> FilteredTokensIterator {
  let Ok(tokens) = get_file_token_list(file_path) else { panic!("Error reading file"); };

  convert_vec_filtered_iter(tokens)
}

pub(crate) fn convert_vec_filtered_iter(tokens: Vec<Token>) -> FilteredTokensIterator {
  if let Some(err) = check_tokens(&tokens) {
    panic!("{err}");
  }

  let is_not_comment: CommentFilter = is_not_comment;

  let token_iter: FilteredTokensIterator = tokens.into_iter()
                                                 .filter(is_not_comment)
                                                 .peekable();

  token_iter
}

pub(crate) fn gen_iter_till_token_or_end(token_iter: &mut FilteredTokensIterator, read_till_token: &Token) -> FilteredTokensIterator {
  let mut tokens: Vec<Token> = vec![];

  let mut seen_open_curl = 0;
  let mut seen_open_paren = 0;
  let mut seen_start_if = 0;
  let mut seen_start_case = 0;
  let mut seen_start_loop = 0;

  // let mut seen_open_curl = if read_till_token.is_same_type(&CLOSE_PAREN_TYPE) { -1 } else {0};
  // let mut seen_open_paren = if read_till_token.is_same_type(&CLOSE_CURL_TYPE) { -1 } else {0};;

  // read tokens, accounting for matching brackets and matching expressions
  loop {
    if peek_token_eq(token_iter, read_till_token) &&
        seen_open_curl == 0 &&
        seen_open_paren == 0 &&
        seen_start_if == 0 &&
        seen_start_case == 0 &&
        seen_start_loop == 0 {
      break; // reached the real end, accounted for all matching brackets
    }

    let token = match token_iter.next() {
      None => {
        assert_eq!(seen_open_curl, 0);
        assert_eq!(seen_open_paren, 0);
        break;
      }
      Some(t) => t,
    };

    match token {
      Token::OpenParen { .. } => seen_open_paren += 1,
      Token::CloseParen { .. } => seen_open_paren -= 1,

      Token::OpenCurl { .. } => seen_open_curl += 1,
      Token::CloseCurl { .. } => seen_open_curl -= 1,

      Token::If { .. } => seen_start_if += 1,
      Token::EndIf { .. } => seen_start_if -= 1,

      Token::Case { .. } => seen_start_case += 1,
      Token::EndCase { .. } => seen_start_case -= 1,

      Token::Loop { .. } => seen_start_loop += 1,
      Token::EndLoop { .. } => seen_start_loop -= 1,

      _ => ()
    }

    tokens.push(token);
  }

  /*  if read_till_token.is_same_type(&CLOSE_CURL_TYPE) || read_till_token.is_same_type(&CLOSE_PAREN_TYPE) {
      let open_bracket_type = if read_till_token.is_same_type(&CLOSE_CURL_TYPE) { OPEN_CURL_TYPE } else { OPEN_PAREN_TYPE };
      let mut seen_matching_brackets = 0;
      
      // read tokens, accounting for matching brackets
      loop {
        if peek_token_eq(token_iter, read_till_token) && seen_matching_brackets == 0 {
          break; // reached the real end, accounted for all matching brackets
        }
        
        let token = match token_iter.next() {
          None => break,
          Some(t) => t, 
        };
        
        if token.is_same_type(&open_bracket_type) {
          seen_matching_brackets += 1;
        } else if token.is_same_type(read_till_token) {
          seen_matching_brackets -= 1;
        }
        
        tokens.push(token);
      }
    } else {
      // reads all tokens till first [`read_till_token`] is seen, returns a new iterator
      tokens = from_fn(|| token_iter.next_if(|token| !token.is_same_type(read_till_token))).collect();
    }
  */

  /*
    if cfg!(test) {
      println!("START: ============================================");
      println!("generate_iter_till_token: peek: {:?}, expected: {:?}", token_iter.peek(), read_till_token);
  
      println!("Item count: {}", tokens.len());
      for t in tokens.clone() {
        println!("{:?}", t);
      }
      println!("END:   ================ {:?} ", read_till_token);
    }
  */
  if token_iter.peek().is_some() {
    // If there are more tokens, then the next token in iterator must match the `read_till_token`;
    // Otherwise we have reached the end of list, no need to assert
    assert!(peek_token_eq(token_iter, read_till_token));
  }

  let iter = convert_vec_filtered_iter(tokens);
  iter
}

fn is_not_comment(token: &Token) -> bool {
  !matches!(token, Token::Comment { .. })
}

pub(crate) fn is_eof(token_iter: &mut FilteredTokensIterator) -> bool {
  matches!(token_iter.peek(), None | Some(Token::EOF))
}

pub(crate) fn consume_if_available(token_iter: &mut FilteredTokensIterator, expected: Token) {
  /*  if cfg!(test) {
      for token in token_iter.clone() {
        println!("consume_if_available: {:?}", token);
      }
    } 
  */
  match token_iter.next() {
    None => return,
    Some(token) => {
      assert!(token.is_same_type(&expected), "Token not matched, expected: {:?} received {:?}", expected, token);
    }
  };
}

pub(crate) fn consume_required(token_iter: &mut FilteredTokensIterator, expected: Token) {
  assert!(token_iter.peek().is_some(), "Unexpected end of token stream");

  match token_iter.next() {
    None => panic!("Error reading token from the iterator"),
    Some(token) if !token.is_same_type(&expected) => panic!("The next token from the iterator does not match expected. Read {:?} Expected {:?}", token, expected),
    _ => ()
  }
}

/// # Panics 
///
/// if [`token_option`] is [`None`]
#[must_use]
pub(crate) fn match_required_token(token_option: Option<Token>, expected: Token) -> Token {
  if let Some(token) = token_option {
    assert!(token.is_same_type(&expected), "Unexpected token: {:?}", token);
    token
  } else {
    panic!("Unexpected EOF");
  }
}

fn check_tokens(tokens: &Vec<Token>) -> Option<String> {
  let mut errors: String = String::new();
  for token in tokens {
    match token {
      Token::Empty => {
        errors.push_str("Empty token! Parsing failed somewhere, can't specify details.\n");
      }
      Token::Error { error_char, line_num, line_pos } => {
        let x = format!("Error on line {line_num} at pos {line_pos}, offending character {error_char}.");
        errors.push_str(x.as_str());
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

#[inline]
pub(crate) fn peek_token_eq(token_iter: &mut FilteredTokensIterator, expected: &Token) -> bool {
  let peeked_token = token_iter.peek();
  match peeked_token {
    Some(token) => token.is_same_type(expected),
    None => expected.is_same_type(&Token::EOF),
  }
}

#[inline]
pub(crate) fn peek_not_eq_or_eof(token_iter: &mut FilteredTokensIterator, expected: &Token) -> bool {
  !is_eof(token_iter) && !peek_token_eq(token_iter, expected)
  /*  
    let peeked_token = token_iter.peek();
    let result = match peeked_token {
      Some(token) => {
        if cfg!(test) {
          println!("peek_token_neq: {:?}, expected: {:?}", token, expected);
        }
        !token.is_same_type(expected)
      }
      
      None => !expected.is_same_type(&Token::EOF), // iterator is at end, check if peeked = eof
    };
    result
  
      if token_iter.peek().is_none() {
      return false;
    }
    let next_token_is_expected_token = peek_token_eq(token_iter, expected);
    !next_token_is_expected_token
  */
}

#[inline]
pub(crate) fn peek_token_in(token_iter: &mut FilteredTokensIterator, expected: &[Token]) -> bool {
  // Read till end of file if `expected` is empty
  if expected.is_empty() {
    return token_iter.peek().is_none(); // check if `token_iter` has reached the EOF 
  }

  let Some(peek) = token_iter.peek() else { return false };

  expected.iter().any(|token| peek.is_same_type(token))
}

#[cfg(test)]
mod test_token {
  use super::*;

  #[test]
  fn test_is_same_type() {
    let test1 = Token::Ident { value: "Test Ident 1".parse().unwrap(), line_num: 10, line_pos: 50 };
    let test2 = Token::Ident { value: "Test other ident".parse().unwrap(), line_num: 25, line_pos: 15 };
    assert!(test1.is_same_type(&test2));
  }

  #[test]
  fn test_filtered_token_iter() {
    let file = "test_resources/cool.cl";
    let iter = get_filtered_token_iter(file);
    for token in iter {
      assert_ne!(discriminant(&token), discriminant(&Token::Comment { value: String::new(), line_num: 0, line_pos: 0 }));
      println!("{:?}", token);
    }
  }

  #[test]
  fn test_match_peek_in_list() {
    let expected_set = [IN_TYPE, CASE_TYPE, IDENT_TYPE];
    let token_list = [IN_TYPE, COMMA_TYPE, LET_TYPE];
    let mut iter = convert_vec_filtered_iter(Vec::from(token_list));

    assert_eq!(peek_token_in(&mut iter, &expected_set), true);
    iter.next(); // remove `In`
    assert_eq!(peek_token_in(&mut iter, &expected_set), false);
  }
}
