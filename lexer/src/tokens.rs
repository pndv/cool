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

#[derive(PartialEq)]
#[derive(Debug)]
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
}

#[derive(PartialEq)]
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
