
#[derive(PartialEq, Debug, Clone)]
pub struct Symbol {
  name: String,
  env: String,  // name of the enclosing environment
  sym_type: String, // type of Symbol (expression, class, feature etc)
}

#[derive(PartialEq, Debug, Clone)]
pub struct SymbolTable {
  symbols: Vec<Symbol>
}

impl SymbolTable {
  fn enter_scope(&mut self) {}
  
  fn exit_scope(&mut self) {}
  
  fn put(symbol: Symbol) {}
  
  fn get(name: &str) -> Option<Symbol> { None }
}

#[derive(PartialEq, Debug, Clone)]
pub struct MethodTable {

}

#[derive(PartialEq, Debug, Clone)]
pub struct ClassTable {

}
