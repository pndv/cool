use std::borrow::Cow;
use std::fmt::{write, Display, Formatter};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
  Empty,
  Eof, //end of file

  Error,
  Comment,

  Ident,

  Dot,
  Comma,

  Assign, // `<-`

  At,
  Tilde,

  Plus,
  Minus,
  Star,
  ForwardSlash,
  LessOrEqual,
  Less,
  Equal,

  Colon,
  SemiColon,

  OpenParen,
  CloseParen,

  OpenCurl,
  CloseCurl,

  Class,
  Inherits,

  If,
  Then,
  Else,
  EndIf,

  While,
  Loop,
  EndLoop,

  Let,
  In,

  Case,
  Of,
  CaseBranch, // `=>`
  EndCase,

  New,
  IsVoid,
  Not,

  Int,
  String,
  True,
  False,

  SelfType,
}

impl Display for TokenType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      TokenType::Empty => write!(f, "Empty"),
      TokenType::Eof => write!(f, "EOF"),
      TokenType::Error => write!(f, "Error"),
      TokenType::Comment => write!(f, "Comment"),
      TokenType::Ident => write!(f, "IDENT"),
      TokenType::Dot => write!(f, "."),
      TokenType::Comma => write!(f, ","),
      TokenType::Assign => write!(f, "<-"),
      TokenType::At => write!(f, "@"),
      TokenType::Tilde => write!(f, "~"),
      TokenType::Plus => write!(f, "+"),
      TokenType::Minus => write!(f, "-"),
      TokenType::Star => write!(f, "*"),
      TokenType::ForwardSlash => write!(f, "/"),
      TokenType::LessOrEqual => write!(f, "<="),
      TokenType::Less => write!(f, "<"),
      TokenType::Equal => write!(f, "="),
      TokenType::Colon => write!(f, ":"),
      TokenType::SemiColon => write!(f, ";"),
      TokenType::OpenParen => write!(f, "("),
      TokenType::CloseParen => write!(f, ")"),
      TokenType::OpenCurl => write!(f, "{{"),
      TokenType::CloseCurl => write!(f, "}}"),
      TokenType::Class => write!(f, "CLASS"),
      TokenType::Inherits => write!(f, "INHERITS"),
      TokenType::If => write!(f, "IF"),
      TokenType::Then => write!(f, "THEN"),
      TokenType::Else => write!(f, "ELSE"),
      TokenType::EndIf => write!(f, "FI"),
      TokenType::While => write!(f, "WHILE"),
      TokenType::Loop => write!(f, "LOOP"),
      TokenType::EndLoop => write!(f, "POOL"),
      TokenType::Let => write!(f, "LET"),
      TokenType::In => write!(f, "IN"),
      TokenType::Case => write!(f, "CASE"),
      TokenType::Of => write!(f, "OF"),
      TokenType::CaseBranch => write!(f, "=>"),
      TokenType::EndCase => write!(f, "END"),
      TokenType::New => write!(f, "NEW"),
      TokenType::IsVoid => write!(f, "isVoid"),
      TokenType::Not => write!(f, "NOT"),
      TokenType::Int => write!(f, "INT"),
      TokenType::String => write!(f, "STRING"),
      TokenType::True => write!(f, "TRUE"),
      TokenType::False => write!(f, "FALSE"),
      TokenType::SelfType => write!(f, "Self"),
    }
  }
}

#[derive(Debug)]
pub struct Token<'a> {
  value: Cow<'a, str>,
  kind: TokenType,
  line: u32,
  pos: u32,
}

impl PartialEq for Token<'_> {
  fn eq(&self, other: &Self) -> bool {
    self.kind == other.kind
  }
}

impl<'a> Token<'a> {
  pub fn get_kind(self) -> &'a TokenType {
    &self.kind
  }
  
  pub fn new(value: Cow<'a, str>, kind: TokenType, line: u32, pos: u32) -> Self {
    Token { value, kind, line, pos }
  }

  pub fn empty() -> Self {
    Token { value: Cow::from(""), kind: TokenType::Empty, line: 0, pos: 0 }
  }
}

impl Display for Token<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{} type {} [ {} ]", self.line, self.pos, self.kind, self.value)
  }
}
