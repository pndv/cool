﻿use std::cmp::PartialEq;
use std::fs::File;
use std::io::{BufReader, Bytes, Read, Result, Seek, SeekFrom};
use std::iter::{Map, Peekable};

use crate::tokens::WhiteSpace::{CarriageReturn, NewLine, Space, Tab};
use crate::tokens::*;
use WhiteSpace::{FormFeed, VerticalTab};
use crate::tokens::Token::LParen;

enum TokenReadState {
  Start,
  SeenInt,
  SeenString,
  SeenSingleComment,
  SeenNewlineInString,
  End,
}

pub fn get_program(file_path: &str) {
  let (mut char_iter, mut line_num, mut line_pos) = get_buf_reader(file_path);

  while let Some(t) = get_next_token(&mut char_iter, &mut line_num, &mut line_pos) {
    
  }
  
}

fn get_next_token(char_iter: &mut Peekable<Map<Bytes<BufReader<File>>, fn(Result<u8>) -> char>>,
                  line_num: &mut u32,
                  line_pos: &mut u32) -> Option<Token> {
  let mut output: Option<Token> = None;
  let mut token: Token = Token::Empty;
  let mut token_state = TokenReadState::Start;
  
  while let Some(c) = char_iter.next() {
    if WhiteSpace::is_whitespace(c) {
      let ws = WhiteSpace::get(c);
      match ws {
        Space | Tab | FormFeed => *line_pos += 1,
        CarriageReturn | NewLine | VerticalTab => {
          if ws == CarriageReturn {
            char_iter.next_if_eq(&'\n'); // consume CRLF as a single next line
          }
          *line_num += 1;
          *line_pos = 0;
        }
      }

      continue;
    }

    *line_pos += 1;
    match c {
      LESS => {
        if char_iter.next_if_eq(&EQUAL).is_some() {
          *line_pos += 1;
          token = Token::LessOrEqual { line_num: *line_num, line_pos: *line_pos };
        } else if char_iter.next_if_eq(&MINUS).is_some()  {
          *line_pos += 1;
          token = Token::Assign { line_num: *line_num, line_pos: *line_pos };
        } else {
          token = Token::Less { line_num: *line_num, line_pos: *line_pos };
        }
        break;
      }
      DOT => { token = Token::Dot { line_num: *line_num, line_pos: *line_pos };  break; }
      COMMA => { token = Token::Comma { line_num: *line_num, line_pos: *line_pos }; break; }
      AT => { token = Token::At { line_num: *line_num, line_pos: *line_pos };  break; }
      TILDE => { token = Token::Tilde { line_num: *line_num, line_pos: *line_pos };  break; }
      STAR => { token = Token::Star { line_num: *line_num, line_pos: *line_pos };  break; }
      FORWARD_SLASH => { token = Token::ForwardSlash { line_num: *line_num, line_pos: *line_pos };   break; }
      PLUS => { token = Token::Plus { line_num: *line_num, line_pos: *line_pos }; break; }
      MINUS => {
        if char_iter.next_if_eq(&MINUS).is_some() {
          *line_pos += 1;
          token = process_single_line_comment(char_iter, line_num, line_pos);
        } else {
          token = Token::Minus { line_num: *line_num, line_pos: *line_pos };
        }
        break;
      }
      EQUAL => { token = Token::Equal { line_num: *line_num, line_pos: *line_pos }; break; }
      COLON => { token = Token::Colon { line_num: *line_num, line_pos: *line_pos }; break; }
      SEMI_COLON => { token = Token::SemiColon { line_num: *line_num, line_pos: *line_pos }; break; }
      RIGHT_PAREN => { token = Token::RParen { line_num: *line_num, line_pos: *line_pos }; break; }
      LEFT_PAREN => {
        if char_iter.next_if_eq(&STAR).is_some() {
          *line_pos += 1;
          token = process_multi_line_comment(char_iter, line_num, line_pos);
        } else {
          token = Token::LParen { line_num: *line_num, line_pos: *line_pos };
        }
        break; 
      }
      LEFT_CURL => { token = Token::LCurl { line_num: *line_num, line_pos: *line_pos }; break; }
      RIGHT_CURL => { token = Token::RCurl { line_num: *line_num, line_pos: *line_pos }; break; }
      DOUBLE_QUOTE => { token = get_string_token(char_iter, line_num, line_pos); break; }

      '0'..='9' => { token = get_int_token(char_iter, line_num, line_pos, c); break; }
      
      'a'..='z' | 'A'..='Z' | '_' => { token = get_ident_token(char_iter, line_num, line_pos, c); break; }

      _ => { panic!("Unexpected char {c} at line {line_num} position {line_pos}") }
    }
  }

  if token != Token::Empty {
    output = Some(token);
  }  
  
  output
}

fn process_single_line_comment(char_iter: &mut Peekable<Map<Bytes<BufReader<File>>, fn(Result<u8>) -> char>>,
                               line_num: &mut u32,
                               line_pos: &mut u32) -> Token {
  let mut comment = String::new();
  let mut token = Token::Comment { comment_value: String::new(), line_num: *line_num, line_pos: *line_pos };
  // Comments are between `--` or `--` and till end of line
  while let Some(c) = char_iter.next() {
    *line_pos += 1;
    
    match c {
      MINUS => {
        if char_iter.next_if_eq(&MINUS).is_some() {
          // comment ends
          *line_pos +=1 ;
          break;
        }
      }
      '\r' | '\n' => {
        if c == '\r' {
          // consume \r\n together
          let _ = char_iter.next_if_eq(&'\n');
        }
        
        *line_pos = 0;
        *line_num += 1;
        
        break;
      }
      _ => {
        comment.push(c);
        continue;
      }
    }
  }

  if let Token::Comment { ref mut comment_value, .. } = token {
    *comment_value = comment;
  }
  
  token
}

fn process_multi_line_comment(char_iter: &mut Peekable<Map<Bytes<BufReader<File>>, fn(Result<u8>) -> char>>,
                               line_num: &mut u32,
                               line_pos: &mut u32) -> Token {
  let mut comment = String::new();
  let mut token = Token::Comment { comment_value: String::new(), line_num: *line_num, line_pos: *line_pos };
  // Comments are between `(*` and `*)` 
  while let Some(c) = char_iter.next() {
    *line_pos += 1;
    
    match c {
      STAR => {
        if char_iter.next_if_eq(&RIGHT_PAREN).is_some() {
          // comment ends
          *line_pos += 1;
          break;
        }
      }
      '\r' | '\n' => {
        comment.push(c);
        if c == '\r' {
          // consume \r\n together
          if let Some(d) = char_iter.next_if_eq(&'\n') {
            comment.push(d)
          }
        }
        
        *line_pos = 0;
        *line_num += 1;
      }
      _ => {
        comment.push(c);
        continue;
      }
    }
  }

  if let Token::Comment { ref mut comment_value, .. } = token {
    *comment_value = comment;
  }
  
  token
}

fn get_ident_token(char_iter: &mut Peekable<Map<Bytes<BufReader<File>>, fn(Result<u8>) -> char>>,
                   line_num: &mut u32,
                   line_pos: &mut u32,
                   initial_ident: char) -> Token {

  let mut token: Token = Token::Empty;

  let mut ident_val = String::from(initial_ident);

  for c in char_iter.by_ref() {
    *line_pos += 1;
    match c {
      'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => ident_val.push(c),
      _ => {
        if c == '\r' {
          char_iter.next_if_eq(&'\n'); // consume \r\n
          *line_pos = 0;
          *line_num += 1;
        } 
        
        break; 
      }
    }
  }
  
  token = Token::Ident {value: ident_val, line_num: *line_num, line_pos: *line_pos};
  token
}

fn get_int_token(char_iter: &mut Peekable<Map<Bytes<BufReader<File>>, fn(Result<u8>) -> char>>,
                 line_num: &mut u32,
                 line_pos: &mut u32, 
                 initial_digit: char) -> Token {

  let mut token: Token = Token::Empty;
  
  let mut int_val = initial_digit as i32 - '0' as i32;
  
  while let Some(c) = char_iter.by_ref().peek() {
    match c { 
     '0'..='9' => {
       let i = char_iter.next().unwrap();
       let t = i as i32 - '0' as i32;
       int_val *= 10;
       int_val += t;
     }
      _ => break,
    }  
  }
  
  token = Token::Int {value: int_val, line_num: *line_num, line_pos: *line_pos};
  token
}

fn get_string_token(char_iter: &mut Peekable<Map<Bytes<BufReader<File>>, fn(Result<u8>) -> char>>,
                    line_num: &mut u32,
                    line_pos: &mut u32) -> Token {
  let mut token_str = String::new();
  while let Some(c) = char_iter.next() {
    *line_pos += 1;
    match c {
      '\0' => {
        return Token::Error {error_char: String::from("Null Character"), line_num: *line_num, line_pos: *line_pos}
      }
      '\\' => { // cover escaped characters
        let p = char_iter.peek();
        assert_ne!(p, None, "Unterminated string");
        let peek = p.unwrap();

        if peek == &'t' {
          char_iter.next(); // consume the 't'
          token_str.push('\t');
        } else if peek == &'n' {
          char_iter.next(); // consume the 'n'
          token_str.push('\n');
        } else if peek == &'r' {
          char_iter.next(); // consume the 'r'
          token_str.push('\r');
        }
      }
      DOUBLE_QUOTE => {
        return Token::Str {
          value: token_str,
          line_num: *line_num,
          line_pos: *line_pos,
        }
      }
      x => {
        token_str.push(x);
      }
    }
  }

  panic!("String terminated incorrectly at line {line_num} position {line_pos}");
}

fn map_result_to_char(result: Result<u8>) -> char {
  match result {
    Ok(read_char) => read_char as char,
    Err(e) => {
      panic!("Error reading file character {e}")
    }
  }
}

fn get_buf_reader(file_path: &str) -> (Peekable<Map<Bytes<BufReader<File>>, fn(Result<u8>) -> char>>, u32, u32) {
  let file_open = File::open(file_path);

  if let Err(e) = file_open {
    panic!("Failed to open file {file_path} with error {e}");
  }

  let mut buf_reader = BufReader::new(file_open.unwrap());

  // Ignore byte order marker, if present. UTF-8 byte-order marker is first 3 bytes of file = [0xEF 0xBB 0xBF]
  let mut read_byte = [0; 3]; // Buffer to hold 3 bytes
  let mut r = buf_reader.read(&mut read_byte);

  if let Err(e) = r {
    panic!("Failed to read file with error {}", e);
  }

  if read_byte != [0xEF, 0xBB, 0xBF] {
    if let Err(e) = buf_reader.seek(SeekFrom::Start(0)) {
      panic!("File starts with UTF-8 Byte Order Marker, which was ignored. \
      Failed to seek to start of file with error {e}");
    }
  }

  let map: Map<Bytes<BufReader<File>>, fn(Result<u8>) -> char> = buf_reader.bytes()
                                                                           .map(map_result_to_char);
  let peekable_buffer: Peekable<Map<Bytes<BufReader<File>>, fn(Result<u8>) -> char>> = map.peekable();

  let line_num = 1;
  let line_pos = 1;

  (peekable_buffer, line_num, line_pos)
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_TEXT_FILE_PATH: &str = "test_resources/test";
  const TEST_COOL_FILE_PATH: &str = "test_resources/cool.cl";

  #[test]
  fn test_buf_reader() {
    let (buf_reader, line_num, line_pos) = get_buf_reader(TEST_TEXT_FILE_PATH);

    for c in buf_reader {
      print!("{c}");
    }
  }

  #[test]
  fn test_tokeniser() {
    // let files = ["test_resources/cool.cl", "test_resources/arith.cl"];
    let files = ["test_resources/arith.cl"];
    
    for file in &files {
      println!("\n\n=== Testing {file} ===\n\n");
      
      let (mut buf_reader, mut line_num, mut line_pos) = get_buf_reader(file);
      
      while let Some(t) = get_next_token(&mut buf_reader, &mut line_num, &mut line_pos) {
        println!("{:?}", t);
        let is_error_token = matches!(t, Token::Error { .. });
        assert!(!is_error_token);
      }
    }

  }
}
