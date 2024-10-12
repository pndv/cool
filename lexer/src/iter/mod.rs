pub(crate) mod char {
  use std::fmt::{Display, Formatter};
  
  /// Represents a single character in a program file with its line number and position
  #[derive(PartialEq, Eq, Debug, Clone, Hash)]
  pub(crate) struct ProgramChar {
    pub(crate) char_at: char,
    pub(crate) line_num: u32,
    pub(crate) line_pos: u32,
  }
  impl Display for ProgramChar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}:{} = {}", self.line_num, self.line_pos, self.char_at)
    }
  }

  impl PartialEq<char> for ProgramChar {
    fn eq(&self, other: &char) -> bool {
      self.char_at == *other
    }
  }

  impl ProgramChar {
    pub(crate) fn is_whitespace(&self) -> bool {
      self.char_at == ' ' ||
          self.char_at == '\t' ||
          self.char_at == '\n' ||
          self.char_at == '\r' ||
          self.char_at == '\u{c}' ||
          self.char_at == '\u{b}'
    }
  }
}
pub(crate) mod file;

pub mod token;
