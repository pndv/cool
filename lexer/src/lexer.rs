use std::cmp::PartialEq;
use std::fs::File;
use std::io::{BufReader, Bytes, Read, Result, Seek, SeekFrom};
use std::iter::{Map, Peekable};

use crate::tokens::WhiteSpace::{CarriageReturn, NewLine, Space, Tab};
use crate::tokens::*;
use WhiteSpace::{FormFeed, VerticalTab};
use crate::tokens::Token::Else;

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
          char_iter.next_if_eq(&'\n'); // consume CRLF as a single next line
          *line_num += 1;
          *line_pos = 0;
        }
      }

      continue;
    }

    match c {
      LESS => {
        if let Some(_) = char_iter.next_if_eq(&EQUAL) {
          token = Token::LessOrEqual { line_num: *line_num, line_pos: *line_pos };
        } else {
          token = Token::Less { line_num: *line_num, line_pos: *line_pos };
        }
        break;
      }
      DOT => { token = Token::Dot { line_num: *line_num, line_pos: *line_pos };break; }
      COMMA => { token = Token::Comma { line_num: *line_num, line_pos: *line_pos };break; }
      AT => { token = Token::At { line_num: *line_num, line_pos: *line_pos }; break; }
      TILDE => { token = Token::Tilde { line_num: *line_num, line_pos: *line_pos }; break; }
      STAR => { token = Token::Star { line_num: *line_num, line_pos: *line_pos }; break; }
      FORWARD_SLASH => { token = Token::ForwardSlash { line_num: *line_num, line_pos: *line_pos }; break; }
      PLUS => { token = Token::Plus { line_num: *line_num, line_pos: *line_pos }; break; }
      MINUS => {
        if let Some(_) = char_iter.next_if_eq(&MINUS){
          // process single line comment
          
          
        } else {
          token = Token::Minus { line_num: *line_num, line_pos: *line_pos };
        }
        break;
      }
      EQUAL => { token = Token::Equal { line_num: *line_num, line_pos: *line_pos }; break; }
      COLON => { token = Token::Colon { line_num: *line_num, line_pos: *line_pos }; break; }
      SEMI_COLON => { token = Token::SemiColon { line_num: *line_num, line_pos: *line_pos }; break; }
      LEFT_PAREN => { token = Token::LParen { line_num: *line_num, line_pos: *line_pos }; break; }
      RIGHT_PAREN => { token = Token::RParen { line_num: *line_num, line_pos: *line_pos }; break; }
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



fn get_ident_token(char_iter: &mut Peekable<Map<Bytes<BufReader<File>>, fn(Result<u8>) -> char>>,
                   line_num: &mut u32,
                   line_pos: &mut u32,
                   initial_ident: char) -> Token {

  let mut token: Token = Token::Empty;

  let mut ident_val = String::from(initial_ident);
  
  while let Some(c) = char_iter.next() {
    match c { 
     'a'..='z' | 'A'..='Z' | '_'  => {
       ident_val.push(c);
     }
      
      ' ' | '\t' => {
        // end of token
        break;
      }
      
      _ => {
        token  = Token::Error {line_num: *line_num, line_pos: *line_pos};
        return token;
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
  
  let mut int_val = initial_digit as i32;
  
  while let Some(c) = char_iter.peek() {
    match c { 
     '0'..='9' => {
       let t = *c as i32;
       int_val *= 10;
       int_val += t;
     }
      
      ' ' | '\t' => {
        // end of token
        break;
      }
      
      _ => {
        token  = Token::Error {line_num: *line_num, line_pos: *line_pos};
        return token;
      }
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
        return Token::Error {line_num: *line_num, line_pos: *line_pos}
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
    let (mut buf_reader, mut line_num, mut line_pos) = get_buf_reader(TEST_COOL_FILE_PATH);

    while let Some(t) = get_next_token(&mut buf_reader, &mut line_num, &mut line_pos) {
      println!("{:?}", t);
    }
  }
}
