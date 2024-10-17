use crate::model::{Ident, Type};

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Formal {
  pub(crate) formal_name: Ident,
  pub(crate) formal_type: Type,
}

impl From<(Ident, Type)> for Formal {
  fn from((formal_name, formal_type): (Ident, Type)) -> Self {
    Self {
      formal_name,
      formal_type,
    }
  }
}
