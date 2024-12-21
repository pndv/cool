use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug, Clone)]
pub enum EnvType {
  Program,
  Class,
  Let,
  If,
  Then,
  Else,
  While,
  Loop,
  Case
}

impl Display for EnvType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      EnvType::Program => write!(f, "Env: PROGRAM"),
      EnvType::Class => write!(f, "Env: CLASS"),
      EnvType::Let => write!(f, "Env: LET"),
      EnvType::If => write!(f, "Env: IF"),
      EnvType::Then => write!(f, "Env: THEN"),
      EnvType::Else => write!(f, "Env: ELSE"),
      EnvType::While => write!(f, "Env: WHILE"),
      EnvType::Loop => write!(f, "Env: LOOP"),
      EnvType::Case => write!(f, "Env: CASE"),
    }
  }
}

#[derive(PartialEq, Debug, Clone)]
pub enum SymbolType {
  Ident{ident_type: String},
  Type,
}

impl Display for SymbolType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      SymbolType::Ident {ident_type} => write!(f, "[[ IDENT : {ident_type}]]"),
      SymbolType::Type => write!(f, "[[ TYPE ]]"),
    }
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Symbol {
  pub name: String,
  pub env_type: EnvType,  // name of the enclosing environment
  pub sym_type: SymbolType, // type of Symbol (Ident/Type)
}

#[derive(PartialEq, Debug, Clone)]
pub struct SymbolTable {
  symbols: Vec<HashMap<String, Symbol>>
}

impl Display for Symbol {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Symbol: {} in Env: {} of type: {}", self.name, self.env_type, self.sym_type)
  }
}

impl SymbolTable {
  pub fn enter_scope(&mut self) {
    self.symbols.push(HashMap::new());
  }
  
  pub fn exit_scope(&mut self) -> Option<HashMap<String, Symbol>> {
    self.symbols.pop()
  }

  pub fn cur_scope(&self) -> Option<&HashMap<String, Symbol>> {
    self.symbols.last()
  }
  
  pub fn put(&mut self, symbol: Symbol) {
    if self.symbols.is_empty() {
      self.symbols.push(HashMap::new());
    }
    let mut scope= self.symbols.last_mut().unwrap();

    scope.insert(symbol.name.clone(), symbol);
  }
  
  pub fn lookup_symbol(&mut self, name: &str) -> Option<Symbol> { todo!() }

  pub fn lookup_symbol_by_env_type(&mut self, name: &str, env_type: EnvType) -> bool {
    for i in (0..self.symbols.len()).rev() {
      match self.symbols[i].get(name) {
        Some(symbol) if symbol.env_type == env_type => return true,
        _ => {}
      }
    }
    false
  }

  pub fn new() -> Self { SymbolTable { symbols: vec![] } }
}

impl From<Vec<HashMap<String, Symbol>>> for SymbolTable {
  fn from(value: Vec<HashMap<String, Symbol>>) -> Self {
    SymbolTable { symbols: value }
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct MethodTable {

}

#[derive(PartialEq, Debug, Clone)]
pub struct ClassTable {

}
