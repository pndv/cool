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
pub const LEFT_PAREN: char = '(';
pub const RIGHT_PAREN: char = ')';
pub const LEFT_CURL: char = '{';
pub const RIGHT_CURL: char = '}';

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

  AssignValue { line_num: u32, line_pos: u32 }, // `<-`
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

  LParen { line_num: u32, line_pos: u32 },
  RParen { line_num: u32, line_pos: u32 },
  LCurl { line_num: u32, line_pos: u32 },
  RCurl { line_num: u32, line_pos: u32 },

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
  pub fn random_comment() -> Token { Token::Comment { comment_value: "".to_string(), line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Str`]
  pub fn random_str() -> Token { Token::Str { value: "".to_string(), line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Ident`]
  pub fn random_ident() -> Token { Token::Ident { value: "".to_string(), line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Int`]
  pub fn random_int() -> Token { Token::Int { value: i32::MAX, line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Dot`]
  pub fn random_dot() -> Token { Token::Dot { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Comma`]
  pub fn random_comma() -> Token { Token::Comma { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::AssignValue`]
  pub fn random_assign_value() -> Token { Token::AssignValue { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Lambda`]
  pub fn random_lambda() -> Token { Token::Lambda { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::At`]
  pub fn random_at() -> Token { Token::At { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Tilde`]
  pub fn random_tilde() -> Token { Token::Tilde { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Star`]
  pub fn random_star() -> Token { Token::Star { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::ForwardSlash`]
  pub fn random_forward_slash() -> Token { Token::ForwardSlash { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Plus`]
  pub fn random_plus() -> Token { Token::Plus { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Minus`]
  pub fn random_minus() -> Token { Token::Minus { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::LessOrEqual`]
  pub fn random_less_or_equal() -> Token { Token::LessOrEqual { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Less`]
  pub fn random_less() -> Token { Token::Less { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Equal`]
  pub fn random_equal() -> Token { Token::Equal { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Colon`]
  pub fn random_colon() -> Token { Token::Colon { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::SemiColon`]
  pub fn random_semi_colon() -> Token { Token::SemiColon { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::LParen`]
  pub fn random_left_paren() -> Token { Token::LParen { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::RParen`]
  pub fn right_paren_type() -> Token { Token::RParen { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::LCurl`]
  pub fn random_left_curl() -> Token { Token::LCurl { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::RCurl`]
  pub fn right_curl_type() -> Token { Token::RCurl { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Class`]
  pub fn random_class() -> Token { Token::Class { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Inherits`]
  pub fn random_inherits() -> Token { Token::Inherits { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::If`]
  pub fn random_if() -> Token { Token::If { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Then`]
  pub fn random_then() -> Token { Token::Then { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Else`]
  pub fn random_else() -> Token { Token::Else { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::EndIf`]
  pub fn random_end_if() -> Token { Token::EndIf { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::While`]
  pub fn random_while() -> Token { Token::While { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Loop`]
  pub fn random_loop() -> Token { Token::Loop { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::EndLoop`]
  pub fn random_end_loop() -> Token { Token::EndLoop { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Let`]
  pub fn random_let() -> Token { Token::Let { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::In`]
  pub fn random_in() -> Token { Token::In { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Case`]
  pub fn random_case() -> Token { Token::Case { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Of`]
  pub fn random_of() -> Token { Token::Of { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::EndCase`]
  pub fn random_end_case() -> Token { Token::EndCase { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::New`]
  pub fn random_new() -> Token { Token::New { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::IsVoid`]
  pub fn random_is_void() -> Token { Token::IsVoid { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::Not`]
  pub fn random_not() -> Token { Token::Not { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::True`]
  pub fn random_true() -> Token { Token::True { line_num: u32::MAX, line_pos: u32::MAX } }

  /// Generates random [`Token::False`]
  pub fn random_false() -> Token { Token::False { line_num: u32::MAX, line_pos: u32::MAX } }
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