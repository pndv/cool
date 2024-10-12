use crate::iter::char::ProgramChar;
use crate::iter::file::CharIter;
use crate::tokens::{Token, AT, CARRIAGE_RETURN, CLOSE_CURL, CLOSE_PAREN, COLON, COMMA, DOT, DOUBLE_QUOTE, EQUAL, FORWARD_SLASH, GREATER_THAN, LESS_THAN, LINE_FEED, MINUS, OPEN_CURL, OPEN_PAREN, PLUS, SEMI_COLON, STAR, TILDE};
use std::fmt::Display;
use std::fs::File;
use std::iter::Peekable;
use std::mem::discriminant;
use std::vec::IntoIter;

#[derive(Debug)]
pub struct TokenIter {
  char_iter: CharIter,
  peeked: Option<Token>,
}

impl From<String> for TokenIter {
  fn from(value: String) -> Self {
    TokenIter { char_iter: CharIter::from(value), peeked: None }
  }
}

impl From<File> for TokenIter {
  fn from(value: File) -> Self {
    TokenIter { char_iter: CharIter::from(value), peeked: None }
  }
}

impl Iterator for TokenIter {
  type Item = Token;
  fn next(&mut self) -> Option<Self::Item> {
    if self.peeked.is_some() {
      let output = self.peeked.clone();
      self.peeked = self.next_token();
      output
    } else {
      let output = self.next_token();
      self.peeked = self.next_token();
      output
    }
  }
}

impl TokenIter {
  /// Returns if there are more tokens to be consumed
  pub fn has_next(&mut self) -> bool {
    self.peeked.is_some()
  }

  /// Returns the result of consuming the next token with [expected]
  pub fn consume_required(&mut self, expected: &Token) -> bool {
    matches!(self.next(), Some(token) if token == *expected)
  }

  /// Consumes next token without returning it, does not throw any error 
  pub fn consume_next(&mut self) {
    let _ = self.next();
  }

  /// Peek next token
  pub fn peek(&mut self) -> Option<&Token> {
    self.peeked.as_ref()
  }

  /// Peek and match the next token with [expected]
  pub fn peek_eq(&mut self, expected: &Token) -> bool {
    match self.peek() {
      Some(t) => discriminant(expected) == discriminant(t),
      _ => false,
    }
  }

  /// Collects tokens till iterator reached [expected] token
  /// Keeps in account nested braces, parenthesis, and other expressions
  ///
  /// # Panics   
  ///
  /// When it reaches the end of stream and the next token is not EOF
  pub fn collect_till(&mut self, read_till_token: &Token) -> Peekable<IntoIter<Token>> {
    let mut tokens: Vec<Token> = vec![];

    let mut seen_open_curl = 0;
    let mut seen_open_paren = 0;
    let mut seen_start_if = 0;
    let mut seen_start_case = 0;
    let mut seen_start_loop = 0;
    let mut seen_start_let = 0; // matches with `in`

    // read tokens, accounting for matching brackets and matching expressions
    loop {
      if self.peek_eq(read_till_token) &&
          seen_open_curl == 0 &&
          seen_open_paren == 0 &&
          seen_start_if == 0 &&
          seen_start_case == 0 &&
          seen_start_loop == 0 &&
          seen_start_let == 0 {
        break; // reached the real end, accounted for all matching brackets
      }

      let token = match self.next() {
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

        Token::Let { .. } => seen_start_let += 1,
        Token::In { .. } => seen_start_let -= 1,

        _ => ()
      }

      tokens.push(token);
    }

    if self.peek().is_some() {
      // If there are more tokens, then the next token in iterator must match the `read_till_token`;
      // Otherwise we have reached the end of list, no need to assert
      assert!(self.peek_eq(read_till_token));
    }

    tokens.into_iter().peekable()
  }

  fn next_token(&mut self) -> Option<Token> {
    let mut output: Option<Token> = None;
    let mut token: Token = Token::Empty;

    while let Some(fc) = self.char_iter.next() {
      if fc.is_whitespace() {
        continue;
      }

      let ProgramChar { char_at, line_num, line_pos } = fc;

      match char_at {
        DOT | COMMA | AT | TILDE | STAR | FORWARD_SLASH | PLUS | COLON | SEMI_COLON | CLOSE_PAREN | OPEN_CURL | CLOSE_CURL =>
          {
            token = Token::from(ProgramChar { char_at, line_num, line_pos });
            break;
          }

        LESS_THAN if self.char_iter.peek_eq(EQUAL) => {
          self.char_iter.consume_next();
          token = Token::LessOrEqual { line_num, line_pos };
          break;
        }
        LESS_THAN if self.char_iter.peek_eq(MINUS) => {
          self.char_iter.consume_next();
          token = Token::Assign { line_num, line_pos };
          break;
        }
        LESS_THAN => {
          token = Token::Less { line_num, line_pos };
          break;
        }

        MINUS if self.char_iter.peek_eq(MINUS) => {
          self.char_iter.consume_next();
          token = self.get_single_line_comment();
          break;
        }
        MINUS => {
          token = Token::Minus { line_num, line_pos };
          break;
        }

        EQUAL if self.char_iter.peek_eq(GREATER_THAN) => {
          self.char_iter.consume_next();
          token = Token::CaseBranch { line_num, line_pos };
          break;
        }
        EQUAL => {
          token = Token::Equal { line_num, line_pos };
          break;
        }

        OPEN_PAREN if self.char_iter.peek_eq(STAR) => {
          self.char_iter.consume_next();
          token = self.get_multi_line_comment();
          break;
        }
        OPEN_PAREN => {
          token = Token::OpenParen { line_num, line_pos };
          break;
        }

        DOUBLE_QUOTE => {
          token = self.get_string();
          break;
        }

        '0'..='9' => {
          token = self.get_int();
          break;
        }

        'a'..='z' | 'A'..='Z' | '_' => {
          token = self.get_ident();
          break;
        }

        c => {
          token = Token::Error { value: format!("Unexpected char: {c}"), line_num, line_pos };
          break;
        }
      }
    }

    if token != Token::Empty {
      output = Some(token);
    }

    output
  }

  fn get_string(&mut self) -> Token {
    let (_, line_num, line_pos) = self.char_iter.get_cur_pos();

    let mut token: Token = Token::String { value: String::new(), line_num, line_pos };
    let Token::String { ref mut value, .. } = token else { unreachable!() };

    loop {
      let Some(ProgramChar { char_at, .. }) = self.char_iter.next() else {
        return Token::Error { value: String::from("Improperly terminated string literal"), line_num, line_pos };
      };

      match char_at {
        '\0' => {
          return Token::Error { value: String::from("Null Character"), line_num, line_pos };
        }

        '\\' if self.char_iter.peek_eq('t') => {
          value.push('\t');
          self.char_iter.consume_next();
        }
        '\\' if self.char_iter.peek_eq('n') => {
          value.push('\n');
          self.char_iter.consume_next();
        }
        '\\' if self.char_iter.peek_eq('r') => {
          value.push('\r');
          self.char_iter.consume_next();
        }
        '\\' if self.char_iter.peek_eq('v') => {
          value.push('\u{c}');
          self.char_iter.consume_next();
        }
        '\\' if self.char_iter.peek_eq('f') => {
          value.push('\u{b}');
          self.char_iter.consume_next();
        }

        DOUBLE_QUOTE => return token,

        other => value.push(other),
      }
    }
  }

  fn get_int(&mut self) -> Token {
    let (initial_digit, line_num, line_pos) = self.char_iter.get_cur_pos();

    let mut int_val = initial_digit as i32 - '0' as i32;

    while let Some(peeked_char) = self.char_iter.peek() {
      match peeked_char {
        '0'..='9' => {
          let Some(ProgramChar { char_at, .. }) = self.char_iter.next() else { unreachable!() };
          let t = char_at as i32 - '0' as i32;
          int_val *= 10;
          int_val += t;
        }
        _ => break,
      }
    }

    Token::Int { value: int_val, line_num, line_pos }
  }

  fn get_single_line_comment(&mut self) -> Token {
    let (_, line_num, line_pos) = self.char_iter.get_cur_pos();
    let mut token = Token::Comment { value: String::new(), line_num, line_pos };
    let Token::Comment { ref mut value, .. } = token else { unreachable!() };

    // Comments are from `--` and either till end of line or end of file
    for ProgramChar { char_at, .. } in self.char_iter.by_ref() {
      match char_at {
        CARRIAGE_RETURN | LINE_FEED => break,
        _ => value.push(char_at),
      }
    }

    token
  }

  fn get_multi_line_comment(&mut self) -> Token {
    let (_, line_num, line_pos) = self.char_iter.get_cur_pos();
    let mut token = Token::Comment { value: String::new(), line_num, line_pos };
    let Token::Comment { ref mut value, .. } = token else { unreachable!() };

    // Comments are between `(*` and `*)`
    while let Some(ProgramChar { char_at, .. }) = self.char_iter.next() {
      match char_at {
        STAR if self.char_iter.peek_eq(CLOSE_PAREN) => break,
        _ => value.push(char_at),
      }
    }

    token
  }

  fn get_ident(&mut self) -> Token {
    let (initial_ident, line_num, line_pos) = self.char_iter.get_cur_pos();
    let mut ident_val = String::from(initial_ident);
    let mut token = Token::Ident { value: ident_val, line_num, line_pos };
    let Token::Ident { ref mut value, .. } = token else { unreachable!() };

    while let Some(peek) = self.char_iter.peek() {
      match peek {
        'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
          let Some(ProgramChar { char_at, .. }) = self.char_iter.next() else { unreachable!() };
          value.push(char_at);
        }
        _ => break,
      }
    }

    if let Some(keyword_token) = token.get_keyword() {
      token = keyword_token;
    }

    token
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_token_iterator() {
    let file1 = File::open("test_resources/programs/cool.cl").unwrap();
    let file2 = File::open("test_resources/programs/arith.cl").unwrap();
    let files = vec![file1, file2];

    for file in files {
      let mut iter = TokenIter::from(file);

      for t in iter {
        println!("{:?}", t);
        let is_error_token = matches!(t, Token::Error { .. });
        assert!(!is_error_token);
      }
    }
  }
}



