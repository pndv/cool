use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug, Clone)]
pub enum EnvType {
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
  Ident,
  Type,
}

impl Display for SymbolType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      SymbolType::Ident => write!(f, "[[ IDENT ]]"),
      SymbolType::Type => write!(f, "[[ TYPE ]]"),
    }
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Symbol {
  pub name: String,
  pub env: EnvType,  // name of the enclosing environment
  pub sym_type: SymbolType, // type of Symbol (expression, class, feature etc)
}

#[derive(PartialEq, Debug, Clone)]
pub struct SymbolTable {
  symbols: Vec<HashMap<String, Symbol>>
}

impl Display for Symbol {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Symbol: {} in Env: {} of type: {}", self.name, self.env, self.sym_type)
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
  
  fn get(name: &str) -> Option<Symbol> { None }

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
