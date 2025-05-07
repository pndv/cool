use crate::models::expression::ExpressionNode;
use crate::models::formals::FormalNode;
use crate::models::Node;
use parser::model::feature::{Attribute, Method, ParseFeature};
use std::borrow::{Borrow, Cow};
use std::fmt::{Display, Formatter};

// #[derive(Clone)]
// pub union FeatureNode {
//     attributes: Vec<AttributeNode>,
//     methods: Vec<MethodNode>,
// }

#[derive(Debug, Clone, PartialEq)]
pub enum FeatureNode {
  Attribute(AttributeNode),
  Method(MethodNode),
}

impl From<ParseFeature> for FeatureNode {
  fn from(value: ParseFeature) -> Self {
    match value {
      ParseFeature::Attribute { attribute } => {
        let attribute = AttributeNode::from(attribute);
        FeatureNode::Attribute(attribute)
      }
      ParseFeature::Method { method } => {
        let method = MethodNode::from(method);
        FeatureNode::Method(method)
      }
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AttributeNode {
  pub name: Cow<'static, str>,
  pub attribute_type: Cow<'static, str>,
  pub exp: Option<ExpressionNode>,
}
impl Node<> for AttributeNode {}

impl From<Attribute> for AttributeNode {
  fn from(value: Attribute) -> Self {
    let expr = match value.expr {
      None => None,
      Some(e) => Some(ExpressionNode::from(e)),
    };
    AttributeNode { name: value.name.0, attribute_type: value.return_type.0, exp: expr }
  }
}
impl Display for AttributeNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}



#[derive(Debug, PartialEq, Clone)]
pub struct MethodNode {
  pub name: Cow<'static, str>,
  pub formals: Vec<FormalNode>,
  pub return_type: Cow<'static, str>,
  pub exp: ExpressionNode,
}

impl From<Method> for MethodNode {
  fn from(value: Method) -> Self {
    todo!()
  }
}
impl Node for MethodNode {}

impl Display for MethodNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
