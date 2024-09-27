pub const END_OF_FILE: char = '\0';
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
  pub fn is_keyword(&self) -> bool {
    matches!(self, Token::Ident {  .. } | Token::Inherits { .. } | Token::If { .. } | Token::Then { .. }
          | Token::Else { .. } | Token::EndIf { .. } | Token::While { .. } | Token::Loop { .. }
          | Token::EndLoop { .. } | Token::Let { .. } | Token::In { .. } | Token::Case { .. }
          | Token::Of { .. } | Token::EndCase { .. } | Token::New { .. } | Token::IsVoid { .. }
          | Token::Not { .. } | Token::True { .. } | Token::False { .. })
  }

  pub fn get_keyword(&self) -> Option<Token> {
    match self {
      Token::Ident {ref value, ref line_num, ref line_pos } => {
        let lower_case = value.to_lowercase();
        let v = lower_case.as_str();
        match v {
          "class" => Some(Token::Class { line_num: *line_num, line_pos: *line_pos }),
          "inherits" => Some(Token::Inherits { line_num: *line_num, line_pos: *line_pos }),

          "if" => Some(Token::If { line_num: *line_num, line_pos: *line_pos }),
          "then" => Some(Token::Then { line_num: *line_num, line_pos: *line_pos }),
          "else" => Some(Token::Else { line_num: *line_num, line_pos: *line_pos }),

          "fi" => Some(Token::EndIf { line_num: *line_num, line_pos: *line_pos }),
          "in" => Some(Token::In { line_num: *line_num, line_pos: *line_pos }),
          "let" => Some(Token::Let { line_num: *line_num, line_pos: *line_pos }),

          "isvoid" => Some(Token::IsVoid { line_num: *line_num, line_pos: *line_pos }),
          "not" => Some(Token::Not { line_num: *line_num, line_pos: *line_pos }),

          "loop" => Some(Token::Loop { line_num: *line_num, line_pos: *line_pos }),
          "pool" => Some(Token::EndLoop { line_num: *line_num, line_pos: *line_pos }),
          "while" => Some(Token::While { line_num: *line_num, line_pos: *line_pos }),

          "case"=> Some(Token::Case { line_num: *line_num, line_pos: *line_pos }),
          "esac" => Some(Token::EndCase { line_num: *line_num, line_pos: *line_pos }),
          "new" => Some(Token::New { line_num: *line_num, line_pos: *line_pos }),
          "of" => Some(Token::Of { line_num: *line_num, line_pos: *line_pos }),
          "false" if value.starts_with('f') => Some(Token::False { line_num: *line_num, line_pos: *line_pos }),
          "true" if value.starts_with('t') => Some(Token::False { line_num: *line_num, line_pos: *line_pos }),
          &_ => None,
        }

      },
      _ => None,
    }
  }
}

#[derive(PartialEq)]
pub enum Keywords {
  Class,
  Inherits,
  If,
  Then,
  Else,
  Fi,
  While,
  Loop,
  Pool,
  Let,
  In,
  Case,
  Of,
  Esac,
  New,
  IsVoid,
  Not,
  True,
  False,
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
  pub fn value(&self) -> char {
    match self {
      WhiteSpace::Space => ' ',
      WhiteSpace::Tab => '\t',
      WhiteSpace::NewLine => '\n',
      WhiteSpace::CarriageReturn => '\r',
      WhiteSpace::FormFeed => '\u{c}', // \f
      WhiteSpace::VerticalTab => '\u{b}', // \v
    }
  }

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
