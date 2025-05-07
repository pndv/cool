use std::collections::HashMap;
use std::fmt::{Debug, Display};

pub mod program_node;
pub mod class_node;



pub(crate) trait Node: Display + Debug {
  fn add_symbol(&mut self, name: String, symbol: impl Node);
  fn get_symbol(&self, name: &str) -> Option<&impl Node>;
}

pub struct ProgramNode {
  class_map: HashMap<String, ClassNode>,
}

pub struct ClassNode {
  name: String,
  parent: Option<String>,
  feature_map: HashMap<String, FeatureNode>,
}

pub struct FeatureNode {
  name: String,
  param_type_map: Option<HashMap<String, String>>,
  return_type_name: String,
}