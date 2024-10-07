use mem::discriminant;
use std::mem;

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

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Token {
  Empty,
  Error { error_char: String, line_num: u32, line_pos: u32 },
  Comment { value: String, line_num: u32, line_pos: u32 },

  Str { value: String, line_num: u32, line_pos: u32 },
  Ident { value: String, line_num: u32, line_pos: u32 },
  Int { value: i32, line_num: u32, line_pos: u32 },

  Dot { line_num: u32, line_pos: u32 },
  Comma { line_num: u32, line_pos: u32 },

  Assign { line_num: u32, line_pos: u32 }, // `<-`
  Lambda { line_num: u32, line_pos: u32 }, // `=>`

  At { line_num: u32, line_pos: u32 },
  Tilde { line_num: u32, line_pos: u32 },
  Star { line_num: u32, line_pos: u32 },
  ForwardSlash { line_num: u32, line_pos: u32 },
  Plus { line_num: u32, line_pos: u32 },
  Minus { line_num: u32, line_pos: u32 },
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

  True { line_num: u32, line_pos: u32 },
  False { line_num: u32, line_pos: u32 },
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

#[cfg(test)]
mod test_token {
  use super::*;

  #[test]
  fn test_is_same_type() {
    let test1 = Token::Ident { value: "Test Ident 1".parse().unwrap(), line_num: 10, line_pos: 50 };
    let test2 = Token::Ident { value: "Test other ident".parse().unwrap(), line_num: 25, line_pos: 15 };
    assert!(test1.is_same_type(&test2));
  }
}
