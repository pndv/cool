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

#[derive(PartialEq, Debug)]
pub enum Token {
  Empty,
  Error { error_char: String, line_num: u32, line_pos: u32 },
  Comment { comment_value: String, line_num: u32, line_pos: u32 },

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
  
  pub fn get_keyword(&self) -> Option<Token> {
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

  pub fn is_same_type(&self, other: &Token) -> bool {
    discriminant(self) == discriminant(other)
  }

  /// Generates random [`Token::Empty`]
  pub fn empty_type() -> Token { Token::Empty }

  /// Generates random [`Token::Error`]
  pub fn err_type() -> Token { Token::Error { error_char: "".to_string(), line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Comment`]
  pub fn comment_type() -> Token { Token::Comment { comment_value: "".to_string(), line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Str`]
  pub fn str_type() -> Token { Token::Str { value: "".to_string(), line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Ident`]
  pub fn ident_type() -> Token { Token::Ident { value: "".to_string(), line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Int`]
  pub fn int_type() -> Token { Token::Int { value: i32::MAX, line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Dot`]
  pub fn dot_type() -> Token { Token::Dot { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Comma`]
  pub fn comma_type() -> Token { Token::Comma { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Assign`]
  pub fn assign_type() -> Token { Token::Assign { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Lambda`]
  pub fn lambda_type() -> Token { Token::Lambda { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::At`]
  pub fn at_type() -> Token { Token::At { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Tilde`]
  pub fn tilde_type() -> Token { Token::Tilde { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Star`]
  pub fn star_type() -> Token { Token::Star { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::ForwardSlash`]
  pub fn random_forward_slash() -> Token { Token::ForwardSlash { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Plus`]
  pub fn random_plus() -> Token { Token::Plus { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Minus`]
  pub fn random_minus() -> Token { Token::Minus { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::LessOrEqual`]
  pub fn less_or_equal_type() -> Token { Token::LessOrEqual { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Less`]
  pub fn less_type() -> Token { Token::Less { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Equal`]
  pub fn equal_type() -> Token { Token::Equal { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Colon`]
  pub fn colon_type() -> Token { Token::Colon { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::SemiColon`]
  pub fn semi_colon_type() -> Token { Token::SemiColon { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::OpenParen`]
  pub fn open_paren_type() -> Token { Token::OpenParen { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::CloseParen`]
  pub fn close_paren_type() -> Token { Token::CloseParen { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::OpenCurl`]
  pub fn open_curl_type() -> Token { Token::OpenCurl { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::CloseCurl`]
  pub fn close_curl_type() -> Token { Token::CloseCurl { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Class`]
  pub fn class_type() -> Token { Token::Class { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Inherits`]
  pub fn inherits_type() -> Token { Token::Inherits { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::If`]
  pub fn if_type() -> Token { Token::If { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Then`]
  pub fn then_type() -> Token { Token::Then { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Else`]
  pub fn else_type() -> Token { Token::Else { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::EndIf`]
  pub fn end_if_type() -> Token { Token::EndIf { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::While`]
  pub fn random_while() -> Token { Token::While { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Loop`]
  pub fn loop_type() -> Token { Token::Loop { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::EndLoop`]
  pub fn end_loop_type() -> Token { Token::EndLoop { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Let`]
  pub fn let_type() -> Token { Token::Let { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::In`]
  pub fn in_type() -> Token { Token::In { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Case`]
  pub fn case_type() -> Token { Token::Case { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Of`]
  pub fn of_type() -> Token { Token::Of { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::EndCase`]
  pub fn end_case_type() -> Token { Token::EndCase { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::New`]
  pub fn new_type() -> Token { Token::New { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::IsVoid`]
  pub fn is_void_type() -> Token { Token::IsVoid { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Not`]
  pub fn not_type() -> Token { Token::Not { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::True`]
  pub fn true_type() -> Token { Token::True { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::False`]
  pub fn false_type() -> Token { Token::False { line_num: u32::MAX, line_pos: u32::MAX } }
}

#[derive(PartialEq, Debug)]
pub enum WhiteSpace {
  Space,
  Tab,
  NewLine,
  CarriageReturn,
  FormFeed,
  VerticalTab,
}

impl WhiteSpace {
  pub fn get(value: char) -> WhiteSpace {
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

  pub fn is_whitespace(value: char) -> bool {
    value == ' ' ||
        value == '\t' ||
        value == '\n' ||
        value == '\r' ||
        value == '\u{c}' ||
        value == '\u{b}'
  }
}

mod test {
  use super::*;

  #[test]
  fn test_is_same_type() {}
}