use std::borrow::Cow;
use std::fmt::{Display, Formatter};

pub mod class;
pub mod expressions;
pub mod feature;
pub mod formal;
pub mod program;

#[derive(PartialEq, Debug, Clone)]
pub struct Ident(pub Cow<'static, str>);
impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ident [ {} ]", self.0)
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
        write!(f, "Type [ {} ]", self.0)
    }
}

impl From<String> for Type {
    fn from(value: String) -> Self {
        Type(Cow::Owned(value))
    }
}

impl Type {
    pub fn get_name(&self) -> String {
        String::from(self.0.as_ref())
    }
}

impl Ident {
    pub fn get_name(&self) -> String {
        String::from(self.0.as_ref())
    }
}
