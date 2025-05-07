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

impl Node for ProgramNode {
  fn get_file_name(&self) -> String {
    todo!()
  }

  fn get_line_number(&self) -> u32 {
    todo!()
  }

  fn get_column_number(&self) -> u32 {
    todo!()
  }

  fn get_symbol(&self, name: &str) -> Option<ClassNode> {
    self.class_map.get(name).cloned()
  }

  fn put_symbol(&mut self, name: &str, symbol: &ClassNode) {
    self.class_map.insert(name.to_string(), symbol.clone()); 
  }

  fn get_symbol_type(&self, name: &str) -> Option<String> {
    todo!()
  }

  fn get_parent(&self) -> Option<Box<dyn Node>> {
    None
  }

  fn decorate_ast(&mut self) {
    todo!()
  }
}
