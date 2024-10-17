use crate::model::expressions::Expression;
use crate::model::formal::Formal;
use crate::model::{Ident, Type};

#[derive(PartialEq, Debug, Clone)]
pub struct Feature {
    pub name: Ident,
    pub formals: Option<Vec<Formal>>,
    pub return_type: Type,
    pub expr: Option<Box<Expression>>,
}

impl From<(Ident, Option<Vec<Formal>>, Type, Box<Expression>)> for Feature {
    fn from(
        (name, formals, return_type, expr): (Ident, Option<Vec<Formal>>, Type, Box<Expression>),
    ) -> Self {
        Feature {
            name,
            formals,
            return_type,
            expr: Some(expr),
        }
    }
}

impl From<(Ident, Type, Box<Expression>)> for Feature {
    fn from((name, return_type, expr): (Ident, Type, Box<Expression>)) -> Self {
        Feature {
            name,
            formals: None,
            return_type,
            expr: Some(expr),
        }
    }
}

impl From<(Ident, Type)> for Feature {
    fn from((name, return_type): (Ident, Type)) -> Self {
        Feature {
            name,
            formals: None,
            return_type,
            expr: None,
        }
    }
}
