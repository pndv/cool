use crate::models::Node;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Default)]
pub struct SymbolTable {
  // The first entry is always the global stock, the list of classes in the Program
  symbol_stack: Vec<HashMap<String, Arc<RwLock<Node>>>>,
}

impl SymbolTable {
  pub fn enter_scope(&mut self) {
    self.symbol_stack.push(Default::default());
  }

  pub fn exit_scope(&mut self) -> Result<bool, String> {
    if self.symbol_stack.len() > 1 {
      self.symbol_stack.pop();
      Ok(true)
    } else {
      Err("Empty symbol table".to_string())
    }
  }

  pub fn lookup_symbol(&self, name: &str) -> Option<Arc<RwLock<Node>>> {
    dbg!("Looking up symbol {}", name);
    for current_scope in self.symbol_stack.iter().rev() {
      if current_scope.contains_key(name) {
        let node = current_scope[name].clone();
        return Some(node);
      }
    }
    None
  }
  
  pub fn insert_symbol(&mut self, name: &str, symbol: Arc<RwLock<Node>>) -> Result<(), String>{
    dbg!("Inserting symbol {}", name);
    let Some(current_scope) = self.symbol_stack.last_mut() else {panic!("No current scope")};
    
    if current_scope.contains_key(name) {
      return Err(String::from(format!("Symbol {} already defined", name)))
    }
    
    current_scope.insert(name.to_string(), symbol);
    Ok(())
  }
    
  pub(super) fn get_global_scope(&self) -> &HashMap<String, Arc<RwLock<Node>>> {
    self.symbol_stack.first().unwrap()
  }

  // For testing only
  pub(super) fn get_current_scope(&self) -> &HashMap<String, Arc<RwLock<Node>>> {
    self.symbol_stack.last().unwrap()
  }
}
