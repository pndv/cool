pub const END_OF_FILE: char = '\0';
pub const DOT: char = '.';
pub const AT: char = '@';
pub const TILDE: char = '~';
pub const STAR: char = '*';
pub const FORWARD_SLASH: char = '/';
pub const PLUS: char = '+';
pub const MINUS: char = '-';
pub const LESS: char = '<';
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
  
  Assign { line_num: u32, line_pos: u32 }, // `<-`
  
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
  Fi { line_num: u32, line_pos: u32 },
  While { line_num: u32, line_pos: u32 },
  Loop { line_num: u32, line_pos: u32 },
  Pool { line_num: u32, line_pos: u32 },
  Let { line_num: u32, line_pos: u32 },
  In { line_num: u32, line_pos: u32 },
  Case { line_num: u32, line_pos: u32 },
  Of { line_num: u32, line_pos: u32 },
  Esac { line_num: u32, line_pos: u32 },
  New { line_num: u32, line_pos: u32 },
  IsVoid { line_num: u32, line_pos: u32 },
  Not { line_num: u32, line_pos: u32 },
  True { line_num: u32, line_pos: u32 },
  False { line_num: u32, line_pos: u32 },
}


pub struct StringToken {
  pub token_value: String,
  pub line_num: u32,
  pub line_pos: u32,
}

pub enum TokenType {
  Empty, /* Signifies null token */

  Program,
  Class,
  Feature,
  Formal,
  Expression,
  StrConst,

  Dot,
  At,
  Tilde,
  Star,
  ForwardSlash,
  Plus,
  Minus,
  LessOrEqual,
  Less,
  Equal,
}

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

pub enum Operators {
  Dot,
  At,
  Tilde,
  Star,
  ForwardSlash,
  Plus,
  Minus,
  LessOrEqual,
  Less,
  Equal,
  
  Assign, // <-
  
  ParenLeft,
  ParentRight,
  
  CurlLeft,
  CurlRight,
  
  Comma,
  Colon
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

pub enum Digit {
  Zero = 0,
  One = 1,
  Two = 2,
  Three = 3,
  Four = 4,
  Five = 5,
  Six = 6,
  Seven = 7,
  Eight = 8,
  Nine = 9,
}

pub enum Alphabet {
  UpperA,
  UpperB,
  UpperC,
  UpperD,
  UpperE,
  UpperF,
  UpperG,
  UpperH,
  UpperI,
  UpperJ,
  UpperK,
  UpperL,
  UpperM,
  UpperN,
  UpperO,
  UpperP,
  UpperQ,
  UpperR,
  UpperS,
  UpperT,
  UpperU,
  UpperV,
  UpperW,
  UpperX,
  UpperY,
  UpperZ,
  LowerA,
  LowerB,
  LowerC,
  LowerD,
  LowerE,
  LowerF,
  LowerG,
  LowerH,
  LowerI,
  LowerJ,
  LowerK,
  LowerL,
  LowerM,
  LowerN,
  LowerO,
  LowerP,
  LowerQ,
  LowerR,
  LowerS,
  LowerT,
  LowerU,
  LowerV,
  LowerW,
  LowerX,
  LowerY,
  LowerZ,
}

impl Alphabet {
  pub fn value(&self) -> char {
    match self {
      Alphabet::UpperA => 'A',
      Alphabet::UpperB => 'B',
      Alphabet::UpperC => 'C',
      Alphabet::UpperD => 'D',
      Alphabet::UpperE => 'E',
      Alphabet::UpperF => 'F',
      Alphabet::UpperG => 'G',
      Alphabet::UpperH => 'H',
      Alphabet::UpperI => 'I',
      Alphabet::UpperJ => 'J',
      Alphabet::UpperK => 'K',
      Alphabet::UpperL => 'L',
      Alphabet::UpperM => 'M',
      Alphabet::UpperN => 'N',
      Alphabet::UpperO => 'O',
      Alphabet::UpperP => 'P',
      Alphabet::UpperQ => 'Q',
      Alphabet::UpperR => 'R',
      Alphabet::UpperS => 'S',
      Alphabet::UpperT => 'T',
      Alphabet::UpperU => 'U',
      Alphabet::UpperV => 'V',
      Alphabet::UpperW => 'W',
      Alphabet::UpperX => 'X',
      Alphabet::UpperY => 'Y',
      Alphabet::UpperZ => 'Z',
      Alphabet::LowerA => 'a',
      Alphabet::LowerB => 'b',
      Alphabet::LowerC => 'c',
      Alphabet::LowerD => 'd',
      Alphabet::LowerE => 'e',
      Alphabet::LowerF => 'f',
      Alphabet::LowerG => 'g',
      Alphabet::LowerH => 'h',
      Alphabet::LowerI => 'i',
      Alphabet::LowerJ => 'j',
      Alphabet::LowerK => 'k',
      Alphabet::LowerL => 'l',
      Alphabet::LowerM => 'm',
      Alphabet::LowerN => 'n',
      Alphabet::LowerO => 'o',
      Alphabet::LowerP => 'p',
      Alphabet::LowerQ => 'q',
      Alphabet::LowerR => 'r',
      Alphabet::LowerS => 's',
      Alphabet::LowerT => 't',
      Alphabet::LowerU => 'u',
      Alphabet::LowerV => 'v',
      Alphabet::LowerW => 'w',
      Alphabet::LowerX => 'x',
      Alphabet::LowerY => 'y',
      Alphabet::LowerZ => 'z',
    }
  }

  pub fn get(value: &char) -> Alphabet {
    match value {
      'A' => Alphabet::UpperA,
      'B' => Alphabet::UpperB,
      'C' => Alphabet::UpperC,
      'D' => Alphabet::UpperD,
      'E' => Alphabet::UpperE,
      'F' => Alphabet::UpperF,
      'G' => Alphabet::UpperG,
      'H' => Alphabet::UpperH,
      'I' => Alphabet::UpperI,
      'J' => Alphabet::UpperJ,
      'K' => Alphabet::UpperK,
      'L' => Alphabet::UpperL,
      'M' => Alphabet::UpperM,
      'N' => Alphabet::UpperN,
      'O' => Alphabet::UpperO,
      'P' => Alphabet::UpperP,
      'Q' => Alphabet::UpperQ,
      'R' => Alphabet::UpperR,
      'S' => Alphabet::UpperS,
      'T' => Alphabet::UpperT,
      'U' => Alphabet::UpperU,
      'V' => Alphabet::UpperV,
      'W' => Alphabet::UpperW,
      'X' => Alphabet::UpperX,
      'Y' => Alphabet::UpperY,
      'Z' => Alphabet::UpperZ,
      'a' => Alphabet::LowerA,
      'b' => Alphabet::LowerB,
      'c' => Alphabet::LowerC,
      'd' => Alphabet::LowerD,
      'e' => Alphabet::LowerE,
      'f' => Alphabet::LowerF,
      'g' => Alphabet::LowerG,
      'h' => Alphabet::LowerH,
      'i' => Alphabet::LowerI,
      'j' => Alphabet::LowerJ,
      'k' => Alphabet::LowerK,
      'l' => Alphabet::LowerL,
      'm' => Alphabet::LowerM,
      'n' => Alphabet::LowerN,
      'o' => Alphabet::LowerO,
      'p' => Alphabet::LowerP,
      'q' => Alphabet::LowerQ,
      'r' => Alphabet::LowerR,
      's' => Alphabet::LowerS,
      't' => Alphabet::LowerT,
      'u' => Alphabet::LowerU,
      'v' => Alphabet::LowerV,
      'w' => Alphabet::LowerW,
      'x' => Alphabet::LowerX,
      'y' => Alphabet::LowerY,
      'z' => Alphabet::LowerZ,

      _ => panic!("Not a whitespace {}", value)
    }
  }
}
