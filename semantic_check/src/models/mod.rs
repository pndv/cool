use crate::models::class_node::ClassNode;
use crate::models::expr::ExprNode;
use crate::models::feature_node::FormalNode;
use feature_node::FeatureNode;
use program_node::ProgramNode;
use std::fmt::{Debug, Display};

pub(crate) mod program_node;
pub(crate)  mod class_node;
pub(crate) mod feature_node;
pub(crate) mod expr;
// 
// pub(crate) trait Node: Debug {
//   fn name(&self) -> &str;
// }


#[derive(Debug)]
pub(crate) enum Node {
  SelfType,
  Program{node: ProgramNode},
  Class{node: ClassNode},
  Feature{node: FeatureNode},
  Formal{node: FormalNode},
  Expr{node: ExprNode},
}

impl Node {
  pub fn name(&self) -> &str {
    match self {
      Node::Program { .. } | Node::Expr {..} => "",
      Node::SelfType => "SELF_TYPE",
      Node::Class { node } => node.name.as_str(),
      Node::Feature { node } => node.name.as_str(),
      Node::Formal {node} => node.name.as_str(),
    }
  }
}

