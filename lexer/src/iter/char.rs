use crate::model::char::ProgramChar;
use crate::model::constants::{AT, CARRIAGE_RETURN, CLOSE_CURL, CLOSE_PAREN, COLON, COMMA, DOT, DOUBLE_QUOTE, EQUAL, FORWARD_SLASH, GREATER_THAN, LESS_THAN, LINE_FEED, MINUS, NULL_CHAR, OPEN_CURL, OPEN_PAREN, PLUS, SEMI_COLON, STAR, TILDE};
use crate::model::token::Token;
use std::fs::File;
use std::io::{BufReader, Bytes, Read, Seek, SeekFrom};
use std::iter::Peekable;

#[derive(Debug)]
pub(crate) struct CharIter {
  bytes_iter: Peekable<Bytes<BufReader<File>>>,
  curr_char: char,
  line_num: u32,
  line_pos: u32,
}

impl From<File> for CharIter {
  fn from(value: File) -> Self {
    let mut buf_reader: BufReader<File> = BufReader::new(value);

    // Ignore byte order marker, if present. UTF-8 byte-order marker is first 3 bytes of file = [0xEF 0xBB 0xBF]
    let mut read_byte = [0; 3]; // Buffer to hold 3 bytes
    let r = buf_reader.read(&mut read_byte);

    if let Err(e) = r {
      panic!("Failed to read file with error {}", e);
    }

    if read_byte != [0xEF, 0xBB, 0xBF] {
      if let Err(e) = buf_reader.seek(SeekFrom::Start(0)) {
        panic!("File starts with UTF-8 Byte Order Marker, which was ignored. \
      Failed to seek to start of file with error {e}");
      }
    }

    let bytes_iter: Peekable<Bytes<BufReader<File>>> = buf_reader.bytes().peekable();

    CharIter { curr_char: NULL_CHAR, bytes_iter, line_num: 1, line_pos: 0 }
  }
}

impl From<String> for CharIter {
  fn from(value: String) -> Self {
    let file = match File::open(&value) {
      Ok(f) => f,
      Err(e) => panic!("Failed to open file {value} with error {e}"),
    };

    CharIter::from(file)
  }
}

impl Iterator for CharIter {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    let mut output: Option<Token> = None;
    let mut token: Token = Token::Empty;

    while let Some(c) = self.next_char() {
      if c.is_whitespace() {
        continue;
      }

      let ProgramChar { char_at, line_num, line_pos } = c;

      match char_at {
        LESS_THAN if self.next_if_eq(EQUAL).is_some() => {
          token = Token::LessOrEqual { line_num, line_pos };
          break;
        }
        LESS_THAN if self.next_if_eq(MINUS).is_some() => {
          token = Token::Assign { line_num, line_pos };
          break;
        }
        LESS_THAN => {
          token = Token::Less { line_num, line_pos };
          break;
        }

        DOT => {
          token = Token::Dot { line_num, line_pos };
          break;
        }
        COMMA => {
          token = Token::Comma { line_num, line_pos };
          break;
        }
        AT => {
          token = Token::At { line_num, line_pos };
          break;
        }
        TILDE => {
          token = Token::Tilde { line_num, line_pos };
          break;
        }
        STAR => {
          token = Token::Star { line_num, line_pos };
          break;
        }
        FORWARD_SLASH => {
          token = Token::ForwardSlash { line_num, line_pos };
          break;
        }
        PLUS => {
          token = Token::Plus { line_num, line_pos };
          break;
        }

        MINUS if self.next_if_eq(MINUS).is_some() => {
          token = self.get_single_line_comment();
          break;
        }
        MINUS => {
          token = Token::Minus { line_num, line_pos };
          break;
        }

        EQUAL if self.next_if_eq(GREATER_THAN).is_some() => {
          token = Token::CaseBranch { line_num, line_pos };
          break;
        }
        EQUAL => {
          token = Token::Equal { line_num, line_pos };
          break;
        }

        COLON => {
          token = Token::Colon { line_num, line_pos };
          break;
        }
        SEMI_COLON => {
          token = Token::SemiColon { line_num, line_pos };
          break;
        }
        CLOSE_PAREN => {
          token = Token::CloseParen { line_num, line_pos };
          break;
        }
        OPEN_PAREN if self.next_if_eq(STAR).is_some() => {
          token = self.get_multi_line_comment();
          break;
        }
        OPEN_PAREN => {
          token = Token::OpenParen { line_num, line_pos };
          break;
        }

        OPEN_CURL => {
          token = Token::OpenCurl { line_num, line_pos };
          break;
        }
        CLOSE_CURL => {
          token = Token::CloseCurl { line_num, line_pos };
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

        _ => { panic!("Unexpected char {c} at line {line_num} position {line_pos}") }
      }
    }

    if token != Token::Empty {
      output = Some(token);
    }

    output
  }
}

impl CharIter {
  fn next_char(&mut self) -> Option<ProgramChar> {
    match self.bytes_iter.next() {
      None => None,

      Some(byte_result) => match byte_result {
        Ok(byte) if (byte as char) == CARRIAGE_RETURN => { // match new line `\r` and `\n`
          if let Some(peek) = self.bytes_iter.peek() {
            match peek {
              Ok(next_byte) if *next_byte as char == LINE_FEED => { let _ = self.bytes_iter.next(); }
              _ => (),
            }
          }
          let char_at = LINE_FEED;
          let next = Some(ProgramChar { char_at, line_num: self.line_num, line_pos: self.line_pos + 1 });

          // reset the line number;
          self.line_num += 1;
          self.line_pos = 0;
          self.curr_char = char_at;

          next
        }

        Ok(byte) if (byte as char) == LINE_FEED => {
          let char_at = byte as char;
          let next = Some(ProgramChar { char_at, line_num: self.line_num, line_pos: self.line_pos + 1 });

          // reset the line number;
          self.line_num += 1;
          self.line_pos = 0;
          self.curr_char = char_at;

          next
        }

        Ok(byte) => {
          let char_at = byte as char;

          self.line_pos += 1;
          self.curr_char = char_at;

          Some(ProgramChar { char_at, line_num: self.line_num, line_pos: self.line_pos })
        }

        Err(e) => {
          dbg!("Failed to read file at position {}:{} with error {}", self.line_num, self.line_pos, e);
          None
        }
      }
    }
  }

  // Returns the current position of iterator in the file, along with the last read character
  pub(crate) fn get_cur_pos(&self) -> (char, u32, u32) {
    (self.curr_char, self.line_num, self.line_pos)
  }

  pub(crate) fn peek(&mut self) -> Option<char> {
    match self.bytes_iter.peek() {
      None => None,
      Some(b) => match b {
        Ok(byte) => Some(*byte as char),
        Err(_) => None,
      }
    }
  }

  /// Check if peeked character in stream equals to [`other`]
  pub(crate) fn peek_eq(&mut self, other: char) -> bool {
    match self.peek() {
      None => false,
      Some(c) => c == other,
    }
  }

  /// Check if peeked character in stream is a digit
  pub(crate) fn peek_is_digit(&mut self) -> bool {
    match self.peek() {
      None => false,
      Some(c) => c >= '0' && c <= '9',
    }
  }

  pub(crate) fn next_if_eq(&mut self, other: char) -> Option<ProgramChar> {
    if self.peek_eq(other) {
      self.next_char()
    } else {
      None
    }
  }

  fn get_single_line_comment(&mut self) -> Token {
    let (_, line_num, line_pos) = self.get_cur_pos();
    let mut token = Token::Comment { value: String::new(), line_num, line_pos };
    let Token::Comment { ref mut value, .. } = token else { unreachable!() }; // mutable reference of `value` in `token`

    // Comments are from `--` and either till end of line or end of file
    while let Some(ProgramChar { char_at, .. }) = self.next_char() {
      match char_at {
        CARRIAGE_RETURN  if self.next_if_eq(LINE_FEED).is_some() => break,
        CARRIAGE_RETURN | LINE_FEED => break,
        _ => value.push(char_at),
      }
    }

    token
  }

  fn get_multi_line_comment(&mut self) -> Token {
    let (_, line_num, line_pos) = self.get_cur_pos();
    let mut token = Token::Comment { value: String::new(), line_num, line_pos };
    let Token::Comment { ref mut value, .. } = token else { unreachable!() }; // mutable reference of `value` in `token`

    // Comments are between `(*` and `*)`
    while let Some(ProgramChar { char_at, .. }) = self.next_char() {
      match char_at {
        STAR if self.next_if_eq(CLOSE_PAREN).is_some() => break,
        _ => value.push(char_at),
      }
    }

    token
  }

  fn get_ident(&mut self) -> Token {
    let (initial_ident, line_num, line_pos) = self.get_cur_pos();
    let mut ident_val = String::from(initial_ident);

    while let Some(peek) = self.peek() {
      match peek {
        'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
          let c = self.next_char().unwrap().char_at;
          ident_val.push(c);
        }
        _ => break,
      }
    }

    let mut token = Token::Ident { value: ident_val, line_num, line_pos };

    if let Some(keyword_token) = token.get_keyword() {
      token = keyword_token;
    }

    token
  }

  fn get_int(&mut self) -> Token {
    let (initial_digit, line_num, line_pos) = self.get_cur_pos();
    let mut int_val = initial_digit as i32 - '0' as i32;

    while self.peek_is_digit() {
      let Some(ProgramChar { char_at, .. }) = self.next_char() else { unreachable!() };
      let t = char_at as i32 - '0' as i32;
      int_val *= 10;
      int_val += t;
    }

    Token::Int { value: int_val, line_num, line_pos }
  }

  fn get_string(&mut self) -> Token {
    let mut token_str = String::new();
    let (_, line_num, line_pos) = self.get_cur_pos();
    let mut cur_line_num = line_num;
    let mut cur_line_pos = line_pos;
    while let Some(c) = self.next_char() {
      let ProgramChar { char_at, line_num: char_line, line_pos: char_pos } = c;
      cur_line_num = char_line;
      cur_line_pos = char_pos;

      match char_at {
        NULL_CHAR => return Token::Error { value: String::from("Null Character"), line_num: cur_line_num, line_pos: cur_line_pos },
        DOUBLE_QUOTE => return Token::String { value: token_str, line_num, line_pos },

        '\\' if self.next_if_eq('t').is_some() => token_str.push('\t'),
        '\\' if self.next_if_eq('n').is_some() => token_str.push('\n'),
        '\\' if self.next_if_eq('r').is_some() => token_str.push('\r'),
        '\\' if self.next_if_eq('f').is_some() => token_str.push('\x0C'),
        '\\' if self.next_if_eq('v').is_some() => token_str.push('\x0B'),
        '\\' if self.next_if_eq('\\').is_some() => token_str.push('\\'), // process '\\' 
        '\\' if self.next_if_eq(DOUBLE_QUOTE).is_some() => token_str.push(DOUBLE_QUOTE),
        // '\\' => token_str.push('\\'),
        x => token_str.push(x),
      }
    }

    Token::Error {
      value: format!("String terminated incorrectly at line {line_num} position {line_pos}"),
      line_num: cur_line_num,
      line_pos: cur_line_pos,
    }
  }
}

#[cfg(test)]
mod test {
  use crate::iter::char::CharIter;
  use crate::model::char::ProgramChar;
  use std::fs::File;
  use std::path::Path;

  #[test]
  fn test_single_line_file_iter() {
    let file = File::open(Path::new("../test_resources/file_iter_read_single_line")).unwrap();
    let mut output = String::new();
    let expected = "this is a test";
    let mut iter = CharIter::from(file);
    while let Some(ProgramChar { char_at, .. }) = iter.next_char() {
      output.push(char_at);
    }

    assert_eq!(expected, output.trim());
  }

  #[test]
  fn test_multi_line_file_iter() {
    let file = File::open(Path::new("../test_resources/file_iter_read_multiline")).unwrap();
    let expected = vec![
      ProgramChar { char_at: 't', line_num: 1, line_pos: 1 },
      ProgramChar { char_at: '\n', line_num: 1, line_pos: 2 },
      ProgramChar { char_at: 'e', line_num: 2, line_pos: 1 },
      ProgramChar { char_at: '\n', line_num: 2, line_pos: 2 },
      ProgramChar { char_at: 's', line_num: 3, line_pos: 1 },
      ProgramChar { char_at: 't', line_num: 3, line_pos: 2 },
      ProgramChar { char_at: 't', line_num: 3, line_pos: 3 },
      ProgramChar { char_at: '\n', line_num: 3, line_pos: 4 },
      ProgramChar { char_at: 't', line_num: 4, line_pos: 1 },
      ProgramChar { char_at: '\n', line_num: 4, line_pos: 2 },
    ];
    let mut output: Vec<ProgramChar> = Vec::new();
    let mut iter = CharIter::from(file);
    while let Some(ch) = iter.next_char() {
      println!("{ch}");
      output.push(ch);
    }

    assert_eq!(expected, output);
  }
}
