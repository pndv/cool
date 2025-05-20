use crate::models::expr::ExprNode;
use std::fmt::Display;

#[derive(Debug, Clone, Default)]
pub struct FeatureNode {
  pub(crate) name: String,
  pub(crate) param_type_map: Option<Vec<FormalNode>>, // if None, it's an Attribute; otherwise, it's a Method
  pub(crate) feature_type: String,
  pub(crate) feature_expr: Option<ExprNode>,
}

#[derive(Debug, Clone, Default)]
pub struct FormalNode {
  pub(crate) name: String,
  pub(crate) formal_type: String,
}

