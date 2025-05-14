use crate::models::class_node::ClassNode;
use crate::models::feature_node::FormalNode;
use feature_node::FeatureNode;
use program_node::ProgramNode;
use std::fmt::{Debug, Display, Formatter};

pub mod program_node;
pub mod class_node;
mod feature_node;
mod expr;
// 
// pub(crate) trait Node: Debug {
//   fn name(&self) -> &str;
// }


#[derive(Debug)]
pub(crate) enum Node {
  Program{node: ProgramNode},
  Class{node: ClassNode},
  Feature{node: FeatureNode},
  Formal{node: FormalNode}
}

impl Node {
  pub fn name(&self) -> &str {
    match self {
      Node::Program { .. } => "",
      Node::Class { node } => node.name.as_str(),
      Node::Feature { node } => node.name.as_str(),
      Node::Formal {node} => node.name.as_str(),
    }
  }
}

impl Display for Node {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Node::Program { node } => write!(f, "{}", node.to_string()),
      Node::Class { node } => write!(f, "{}", node.to_string()),
      Node::Feature { node } => write!(f, "{}", node.to_string()),
      Node::Formal { node } => write!(f, "{}", node.to_string()),
    }
  }
}
