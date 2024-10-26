use crate::models::class::ClassNode;
use crate::models::Node;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct ProgramNode {
  pub(crate) classes: Vec<ClassNode>,
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
