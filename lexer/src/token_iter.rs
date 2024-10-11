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

struct FileChar {
  file_char: char,
  line_num: u32,
  line_pos: u32,
}

struct FileCharIter {
  bytes_iter: Peekable<Bytes<BufReader<File>>>,
  line_num: u32,
  line_pos: u32,
}

impl From<File> for FileCharIter {
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

    FileCharIter { bytes_iter, line_num: 1, line_pos: 1 }
  }
}

impl From<String> for FileCharIter {
  fn from(value: String) -> Self {
    let file = match File::open(&value) {
      Ok(f) => f,
      Err(e) => panic!("Failed to open file {value} with error {e}"),
    };

    FileCharIter::from(file)
  }
}

impl Iterator for FileCharIter {
  type Item = FileChar;

  fn next(&mut self) -> Option<Self::Item> {
    match self.bytes_iter.next() {
      None => None,
      Some(byte_result) => match byte_result {
        Ok(byte) => {
          self.line_num += 1;
          self.line_pos += 1;
          let file_char = byte as char;
          
          Some(FileChar {file_char, line_num: self.line_num, line_pos: self.line_pos })
        },
        Err(e) => panic!("Failed to read file at position {}:{} with error {}", self.line_num, self.line_pos, e),
      }
    }
  }
}

#[cfg(test)]
mod test {
  use std::fs::File;
  use std::path::Path;

  #[test]
  fn test_file_iter() {
    let file = File::open(Path::new("test_resources/test")).unwrap();
    let output = String::new();
    let expected = "this is a test";
  }
}