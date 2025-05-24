use crate::models::expr::ExprNode;
use crate::symbol_table::SymbolTable;
use std::fmt::Display;

#[derive(Debug, Clone, Default)]
pub(crate) struct FeatureNode {
  pub(crate) name: String,
  pub(crate) param_type_map: Option<Vec<FormalNode>>, // if None, it's an Attribute; otherwise, it's a Method
  pub(crate) feature_type: String,
  pub(crate) feature_expr: Option<ExprNode>,
}

impl FeatureNode {
  pub(crate) fn get_feature_type(&self) -> String {
    self.feature_type.clone()
  }

  pub(crate) fn set_feature_type(&mut self, feature_type: String) {
    self.feature_type = feature_type;
  }

  pub(crate) fn check_expression(&self, symbol_table: &mut SymbolTable) -> Result<(), String> {
    if self.feature_expr.is_none() {
      return Ok(());
    }

    let mut errors: Vec<String> = Vec::new();
    symbol_table.enter_scope(self.name.clone());

    // Put the formal parameters into the symbol table in the current scope
    if self.param_type_map.is_some() {
      let formals = self.param_type_map.as_ref().unwrap().clone();
      for formal in formals {
        match symbol_table.lookup_symbol(formal.formal_type.as_str()) {
          None => { errors.push(format!("Type {} not found in function {} for parameter {}", formal.formal_type, self.name, formal.name)) }
          Some(node) => {
            match symbol_table.insert_symbol(formal.name.as_str(), node.clone()) {
              Ok(_) => {}
              Err(e) => errors.push(format!("Error in inserting symbol {}. Error: {}", formal.name, e)),
            }
          }
        }
      }
    }
    
    let Some(expr) = self.feature_expr.as_ref();
    
    
    match symbol_table.exit_scope() {
      Ok(_) => {}
      Err(e) => errors.push(e),
    }
    
    if errors.len() > 0 {
      return Err(errors.join("\n"));
    }
    Ok(())
  }
}

#[derive(Debug, Clone, Default)]
pub struct FormalNode {
  pub(crate) name: String,
  pub(crate) formal_type: String,
}

