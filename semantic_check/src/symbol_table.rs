use crate::models::Node;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, PartialEq)]
pub(super) struct Symbol {
  pub (super) name: String,
  pub (super) symbol_type: String,
}

#[derive(Debug)]
pub(super) struct Scope {
  pub name: String,
  pub symbol_stack: HashMap<String, Arc<RwLock<Node>>>,
}

impl Default for Scope {
  fn default() -> Self {
    Scope{name: String::new(), symbol_stack: HashMap::new()}
  }
}


#[derive(Debug)]
pub struct SymbolTable {
  // The first entry is always the global stock, the list of classes in the Program
  scopes: Vec<Scope>,
  // symbol_stack: Vec<HashMap<String, Arc<RwLock<Node>>>>,
}

impl Default for SymbolTable {
  fn default() -> Self {
    Self {
      scopes: vec![],
      // symbol_stack: vec![],
    }
  }
}

impl SymbolTable {
  pub fn enter_scope(&mut self, scope_name: String) {
    //todo: add scope_type and use that as well
    self.scopes.push(Scope {name: scope_name, symbol_stack: HashMap::new()});
    // self.symbol_stack.push(Default::default());
  }

  pub fn exit_scope(&mut self) -> Result<bool, String> {
    if self.scopes.len() > 1 {
      self.scopes.pop();
      Ok(true)
    } else {
      Err("Empty symbol table".to_string())
    }
  }

  pub fn lookup_symbol(&self, name: &str) -> Option<Arc<RwLock<Node>>> {
    // dbg!("Looking up symbol {}", name);


    // Handle `SELF_TYPE` in the symbol table
    if name.eq("SELF_TYPE") {
      // return current class
      let Some(current_scope) = self.scopes.last() else {panic!("No current scope")};
      return self.lookup_symbol(current_scope.name.as_str())
    }

    for current_scope in self.scopes.iter().rev() {
      if current_scope.symbol_stack.contains_key(name) {
        let node = current_scope.symbol_stack[name].clone();
        return Some(node);
      }
    }
    
    dbg!(name == "SELF_TYPE");
    dbg!(name.eq("SELF_TYPE"));
    
    None
  }
  
  pub fn insert_symbol(&mut self, name: &str, symbol: Arc<RwLock<Node>>) -> Result<(), String>{
    if name.eq("SELF_TYPE") || name.is_empty() {
      dbg!("Inserting SELF_TYPE");
    }
    
    // dbg!("Inserting symbol {}", name);
    let Some(current_scope) = self.scopes.last_mut() else {panic!("No current scope")};
    
    if current_scope.symbol_stack.contains_key(name) {
      return Err(String::from(format!("Symbol {} already defined", name)))
    }
    
    current_scope.symbol_stack.insert(name.to_string(), symbol);
    Ok(())
  }
    
  pub(super) fn get_global_scope(&self) -> &HashMap<String, Arc<RwLock<Node>>> {
    &self.scopes.first().unwrap().symbol_stack
  }

  // For testing only
  pub(super) fn get_current_scope(&self) -> &HashMap<String, Arc<RwLock<Node>>> {
    &self.scopes.last().unwrap().symbol_stack
  }
}
