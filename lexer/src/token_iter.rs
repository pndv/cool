use crate::tokens::Token;
use std::fs::File;
use std::io::{BufReader, Bytes, Read, Seek, SeekFrom};
use std::iter::Peekable;

struct TokenIter {
  bytes_iter: Bytes<BufReader<File>>,
  line_num: u32,
  line_pos: u32,
}

impl From<String> for TokenIter {
  fn from(value: String) -> Self {
    let file = match File::open(&value) {
      Ok(f) => f,
      Err(e) => panic!("Failed to open file {value} with error {e}"),
    };

    let mut buf_reader: BufReader<File> = BufReader::new(file);

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

    let bytes_iter = buf_reader.bytes();

    TokenIter { bytes_iter, line_num: 1, line_pos: 1 }
  }
}

impl Iterator for TokenIter {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    todo!()
  }
}

struct CharIter {
  bytes_iter: Peekable<Bytes<BufReader<File>>>,
  line_num: u32,
  line_pos: u32,
}

impl From<String> for CharIter {
  fn from(value: String) -> Self {
    let file = match File::open(&value) {
      Ok(f) => f,
      Err(e) => panic!("Failed to open file {value} with error {e}"),
    };

    let mut buf_reader: BufReader<File> = BufReader::new(file);

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

    CharIter { bytes_iter, line_num: 1, line_pos: 1 }
  }
}

impl Iterator for CharIter {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    match self.bytes_iter.next() {
      None => None,
      Some(byte_result) => match byte_result {
        Ok(byte) => Some(byte as char),
        Err(e) => panic!("Failed to read file at position {}:{} with error {}", self.line_num, self.line_pos, e),
      }
    }
  }
}
