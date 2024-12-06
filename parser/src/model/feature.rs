use crate::model::expressions::Expression;
use crate::model::formal::Formal;
use crate::model::{Ident, Type};

#[derive(PartialEq, Debug, Clone)]
pub enum ParseFeature {
  Attribute { attribute: Attribute },
  Method { method: Method },
}

#[derive(PartialEq, Debug, Clone)]
pub struct Attribute {
  pub name: Ident,
  pub return_type: Type,
  pub expr: Option<Box<Expression>>,
}

impl From<(Ident, Type, Option<Box<Expression>>)> for Attribute {
  fn from((name, return_type, expr): (Ident, Type, Option<Box<Expression>>)) -> Self {
    Self { name, return_type, expr }
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Method {
  pub name: Ident,
  pub formals: Option<Vec<Formal>>,
  pub return_type: Type,
  pub expr: Box<Expression>,
}

impl From<(Ident, Option<Vec<Formal>>, Type, Box<Expression>)> for Method {
  fn from((name, formals, return_type, expr): (Ident, Option<Vec<Formal>>, Type, Box<Expression>)) -> Self {
    Self { name, formals, return_type, expr }
  }
}

impl From<(Ident, Option<Vec<Formal>>, Type, Box<Expression>)> for ParseFeature {
  fn from((name, formals, return_type, expr): (Ident, Option<Vec<Formal>>, Type, Box<Expression>)) -> Self {
    let method = Method { name, formals, return_type, expr };
    ParseFeature::Method { method }
  }
}

impl From<(Ident, Type, Box<Expression>)> for ParseFeature {
  fn from((name, return_type, expr): (Ident, Type, Box<Expression>)) -> Self {
    let method = Method { name, formals: None, return_type, expr };
    ParseFeature::Method { method }
  }
}

impl From<(Ident, Type)> for ParseFeature {
  fn from((name, return_type): (Ident, Type)) -> Self {
    let attribute = Attribute { name, return_type, expr: None };
    ParseFeature::Attribute { attribute }
  }
}
