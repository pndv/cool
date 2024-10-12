use crate::iter::char::ProgramChar;
use crate::tokens::{CARRIAGE_RETURN, LINE_FEED, NULL_CHAR};
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
  type Item = ProgramChar;

  fn next(&mut self) -> Option<Self::Item> {
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
}

impl CharIter {
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

  /// Consumes the next character without returning it, does not throw error if the stream is empty 
  pub(crate) fn consume_next(&mut self) {
    let _ = self.next();
  }
}

#[cfg(test)]
mod test {
  use crate::iter::char::ProgramChar;
  use std::fs::File;
  use std::path::Path;

  #[test]
  fn test_single_line_file_iter() {
    let file = File::open(Path::new("test_resources/file_iter_read_single_line")).unwrap();
    let mut output = String::new();
    let expected = "this is a test";
    let iter = crate::iter::file::CharIter::from(file);
    for char in iter {
      output.push(char.char_at);
    }

    assert_eq!(expected, output.trim());
  }

  #[test]
  fn test_multi_line_file_iter() {
    let file = File::open(Path::new("test_resources/file_iter_read_multiline")).unwrap();
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
    let iter = crate::iter::file::CharIter::from(file);
    for char in iter {
      println!("{char}");
      output.push(char);
    }

    assert_eq!(expected, output);
  }
}
