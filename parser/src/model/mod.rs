use std::borrow::Cow;
use std::fmt::{Display, Formatter};

pub(crate) mod expressions;
pub(crate) mod class;
pub(crate) mod feature;
pub(crate) mod formal;
pub(crate) mod program;

#[derive(PartialEq, Debug, Clone)]
pub struct Ident(pub Cow<'static, str>);
impl Display for Ident {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Ident {}", self.0)
  }
}

impl From<String> for Ident {
  fn from(value: String) -> Self {
    Ident(Cow::Owned(value))
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Type(pub Cow<'static, str>);

impl Display for Type {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Type {}", self.0)
  }
}


impl From<String> for Type {
  fn from(value: String) -> Self {
    Type(Cow::Owned(value))
  }
}
