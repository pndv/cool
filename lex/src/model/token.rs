use crate::model::constants::*;
use std::fmt::{Display, Formatter};
use std::mem::discriminant;

#[derive(Eq, Debug, Clone, Hash)]
pub enum Token {
  Empty,
  EOF, //end of file
  Error { value: String, line_num: u32, line_pos: u32 },
  Comment { value: String, line_num: u32, line_pos: u32 },

  Ident { value: String, line_num: u32, line_pos: u32 },

  Dot { line_num: u32, line_pos: u32 },
  Comma { line_num: u32, line_pos: u32 },

  Assign { line_num: u32, line_pos: u32 }, // `<-`
  CaseBranch { line_num: u32, line_pos: u32 }, // `=>`

  At { line_num: u32, line_pos: u32 },
  Tilde { line_num: u32, line_pos: u32 },

  Plus { line_num: u32, line_pos: u32 },
  Minus { line_num: u32, line_pos: u32 },
  Star { line_num: u32, line_pos: u32 },
  ForwardSlash { line_num: u32, line_pos: u32 },

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

  Int { value: i32, line_num: u32, line_pos: u32 },
  String { value: String, line_num: u32, line_pos: u32 },
  True { line_num: u32, line_pos: u32 },
  False { line_num: u32, line_pos: u32 },

  SelfType { line_num: u32, line_pos: u32 },
}

impl PartialEq for Token {
  fn eq(&self, other: &Self) -> bool {
    discriminant(self) == discriminant(other)
  }
}

impl Display for Token {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Token::Empty => write!(f, "Empty"),
      Token::EOF => write!(f, "EOF"),

      Token::Error { value, line_num, line_pos } => write!(f, "{}:{} type Error [ {} ]", line_num, line_pos, value),
      Token::Comment { value, line_num, line_pos } => write!(f, "{}:{} type Comment [ {} ]", line_num, line_pos, value),
      Token::Ident { value, line_num, line_pos } => write!(f, "{}:{} type Ident [ {} ]", line_num, line_pos, value),
      Token::String { value, line_num, line_pos } => write!(f, "{}:{} type String [ {} ]", line_num, line_pos, value),

      Token::Int { value, line_num, line_pos } => write!(f, "{}:{} type Int [ {} ]", line_num, line_pos, value),

      Token::Dot { line_num, line_pos } => write!(f, "{}:{} type: Dot", line_num, line_pos),
      Token::Comma { line_num, line_pos } => write!(f, "{}:{} type: Comma", line_num, line_pos),
      Token::Assign { line_num, line_pos } => write!(f, "{}:{} type: Assign", line_num, line_pos),
      Token::CaseBranch { line_num, line_pos } => write!(f, "{}:{} type: Lambda", line_num, line_pos),
      Token::At { line_num, line_pos } => write!(f, "{}:{} type: At", line_num, line_pos),
      Token::Tilde { line_num, line_pos } => write!(f, "{}:{} type: Tilde", line_num, line_pos),
      Token::Plus { line_num, line_pos } => write!(f, "{}:{} type: Plus", line_num, line_pos),
      Token::Minus { line_num, line_pos } => write!(f, "{}:{} type: Minus", line_num, line_pos),
      Token::Star { line_num, line_pos } => write!(f, "{}:{} type: Star", line_num, line_pos),
      Token::ForwardSlash { line_num, line_pos } => write!(f, "{}:{} type: ForwardSlash", line_num, line_pos),
      Token::LessOrEqual { line_num, line_pos } => write!(f, "{}:{} type: LessOrEqual", line_num, line_pos),
      Token::Less { line_num, line_pos } => write!(f, "{}:{} type: Less", line_num, line_pos),
      Token::Equal { line_num, line_pos } => write!(f, "{}:{} type: Equal", line_num, line_pos),
      Token::Colon { line_num, line_pos } => write!(f, "{}:{} type: Colon", line_num, line_pos),
      Token::SemiColon { line_num, line_pos } => write!(f, "{}:{} type: SemiColon", line_num, line_pos),
      Token::OpenParen { line_num, line_pos } => write!(f, "{}:{} type: OpenParen", line_num, line_pos),
      Token::CloseParen { line_num, line_pos } => write!(f, "{}:{} type: CloseParen", line_num, line_pos),
      Token::OpenCurl { line_num, line_pos } => write!(f, "{}:{} type: OpenCurl", line_num, line_pos),
      Token::CloseCurl { line_num, line_pos } => write!(f, "{}:{} type: CloseCurl", line_num, line_pos),
      Token::Class { line_num, line_pos } => write!(f, "{}:{} type: Class", line_num, line_pos),
      Token::Inherits { line_num, line_pos } => write!(f, "{}:{} type: Inherits", line_num, line_pos),
      Token::If { line_num, line_pos } => write!(f, "{}:{} type: If", line_num, line_pos),
      Token::Then { line_num, line_pos } => write!(f, "{}:{} type: Then", line_num, line_pos),
      Token::Else { line_num, line_pos } => write!(f, "{}:{} type: Else", line_num, line_pos),
      Token::EndIf { line_num, line_pos } => write!(f, "{}:{} type: EndIf", line_num, line_pos),
      Token::While { line_num, line_pos } => write!(f, "{}:{} type: While", line_num, line_pos),
      Token::Loop { line_num, line_pos } => write!(f, "{}:{} type: Loop", line_num, line_pos),
      Token::EndLoop { line_num, line_pos } => write!(f, "{}:{} type: EndLoop", line_num, line_pos),
      Token::Let { line_num, line_pos } => write!(f, "{}:{} type: Let", line_num, line_pos),
      Token::In { line_num, line_pos } => write!(f, "{}:{} type: In", line_num, line_pos),
      Token::Case { line_num, line_pos } => write!(f, "{}:{} type: Case", line_num, line_pos),
      Token::Of { line_num, line_pos } => write!(f, "{}:{} type: Of", line_num, line_pos),
      Token::EndCase { line_num, line_pos } => write!(f, "{}:{} type: EndCase", line_num, line_pos),
      Token::New { line_num, line_pos } => write!(f, "{}:{} type: New", line_num, line_pos),
      Token::IsVoid { line_num, line_pos } => write!(f, "{}:{} type: IsVoid", line_num, line_pos),
      Token::Not { line_num, line_pos } => write!(f, "{}:{} type: Not", line_num, line_pos),
      Token::True { line_num, line_pos } => write!(f, "{}:{} type: True", line_num, line_pos),
      Token::False { line_num, line_pos } => write!(f, "{}:{} type: False", line_num, line_pos),
      Token::SelfType { line_num, line_pos } => write!(f, "{}:{} type: SelfType", line_num, line_pos),
    }
  }
}

impl Token {
  
  pub fn get_pos(&self) -> (u32, u32) {
    match self {
      Token::Empty | Token::EOF => (0,0),
      Token::Error { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Comment { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Ident { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Dot { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Comma { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Assign { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::CaseBranch { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::At { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Tilde { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Plus { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Minus { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Star { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::ForwardSlash { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::LessOrEqual { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Less { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Equal { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Colon { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::SemiColon { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::OpenParen { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::CloseParen { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::OpenCurl { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::CloseCurl { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Class { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Inherits { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::If { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Then { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Else { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::EndIf { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::While { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Loop { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::EndLoop { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Let { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::In { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Case { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Of { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::EndCase { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::New { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::IsVoid { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Not { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::Int { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::String { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::True { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::False { line_num, line_pos, .. } => (*line_num, *line_pos),
      Token::SelfType { line_num, line_pos, .. } => (*line_num, *line_pos),
    }
  }
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
          KEYWORD_SELF_TYPE => Some(Token::SelfType { line_num: *line_num, line_pos: *line_pos }),
          KEYWORD_FALSE if value.starts_with('f') => Some(Token::False { line_num: *line_num, line_pos: *line_pos }),
          KEYWORD_TRUE if value.starts_with('t') => Some(Token::True { line_num: *line_num, line_pos: *line_pos }),

          &_ => None,
        }
      }
      _ => None,
    }
  }
}


#[cfg(test)]
mod token_test {
  use crate::model::constants::IDENT_TYPE;
  use crate::model::token::Token;

  #[test]
  fn test_equality() {
    let token1 = Token::Ident { value: String::from("Test1"), line_pos: 15, line_num: 40 };
    let token2 = Token::Ident { value: String::from("Test2"), line_pos: 25, line_num: 30 };
    
    assert!(token1 == token2);
    assert!(token1 == IDENT_TYPE);
  }
}
