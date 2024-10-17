use crate::model::expressions::Expression;
use crate::model::formal::Formal;
use crate::model::{Ident, Type};

#[derive(PartialEq, Debug, Clone)]
pub struct Feature {
  pub feature_name: Ident,
  pub formals: Option<Vec<Formal>>,
  pub return_type: Type,
  pub expr: Option<Box<Expression>>,
}

impl From<(Ident, Option<Vec<Formal>>, Type, Box<Expression>)> for Feature {
  fn from((feature_name, formals, return_type, expr): (Ident, Option<Vec<Formal>>, Type, Box<Expression>)) -> Self {
    Feature {
      feature_name,
      formals,
      return_type,
      expr: Some(expr),
    }
  }
}

impl From<(Ident, Type, Box<Expression>)> for Feature {
  fn from((feature_name, return_type, expr): (Ident, Type, Box<Expression>)) -> Self {
    Feature {
      feature_name,
      formals: None,
      return_type,
      expr: Some(expr),
    }
  }
}

impl From<(Ident, Type)> for Feature {
  fn from((feature_name, return_type): (Ident, Type)) -> Self {
    Feature {
      feature_name,
      formals: None,
      return_type,
      expr: None,
    }
  }
}
