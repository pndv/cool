use crate::models::class::ClassNode;
use crate::models::Node;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct ProgramNode {
  pub(crate) classes: Vec<ClassNode>,
  pub(crate) class_map: HashMap<String, ClassNode>,
}

impl Display for ProgramNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut str_class = String::new();
    for class in &self.classes {
      str_class.push_str(format!("{class}").as_str());  
      str_class.push_str("\n");
    }
    
    write!(f, "{}", str_class.trim())
  }
}

impl Node for ProgramNode {}
