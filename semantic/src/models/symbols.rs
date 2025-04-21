use parser::model::class::{BOOL_CLASS_NAME, INT_CLASS_NAME, STR_CLASS_NAME, VOID_CLASS_NAME};
use parser::model::expressions::Expression;
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
  Case,
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

pub type SymbolType = String;

// pub enum SymbolType {
//   Ident{ident_type: String},
//   Type,
// }

// impl Display for SymbolType {
//   fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//     match self {
//       SymbolType::Ident {ident_type} => write!(f, "[[ IDENT : {ident_type}]]"),
//       SymbolType::Type => write!(f, "[[ TYPE ]]"),
//     }
//   }
// }

#[derive(PartialEq, Debug, Clone)]
pub struct Symbol {
  pub name: String,
  pub env_type: EnvType,  // name of the enclosing environment
  pub sym_type: SymbolType, // type of Symbol (Ident/Type)
  pub ret_type: SymbolType, // type of Symbol (Ident/Type)
}

#[derive(PartialEq, Debug, Clone)]
pub struct SymbolTable {
  symbols: Vec<HashMap<String, Symbol>>,
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
    let mut scope = self.symbols.last_mut().unwrap();

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

  pub fn get_symbol_type(expr: Expression) -> Result<SymbolType, String> {
    let str_expr_type = expr.get_type();
    let expr_type: Result<SymbolType, String> = match expr {
      Expression::PartialAssign { .. } | Expression::PartialDispatch { .. } | Expression::PartialCastDispatch { .. } | Expression::PartialBinary { .. } => panic!("Cannot generate expression type for partial expression {expr:?}"),

      Expression::Plus { left, right } | Expression::Minus { left, right } | Expression::Multiply { left, right } | Expression::Divide { left, right } | Expression::LessThan { left, right } | Expression::Equal { left, right } | Expression::LessThanOrEqual { left, right } => {
        let left_expr = Self::get_symbol_type(*left);
        let right_expr = Self::get_symbol_type(*right);

        if left_expr.is_err() || right_expr.is_err() {
          let mut err_msg = String::from("Error in getting sub-expression type.");
          if let Err(e) = left_expr {
            err_msg.push_str(&format!(" Left sub-expression error: {e}"));
          }
          if let Err(e) = right_expr {
            err_msg.push_str(&format!(" Right sub-expression error: {e}"));
          }
          return Err(err_msg);
        }

        let Ok(left_expr_type) = left_expr;
        let Ok(right_expr_type) = right_expr;

        if left_expr_type == INT_CLASS_NAME && right_expr_type == INT_CLASS_NAME {
          match str_expr_type.as_str() {
            "Plus" | "Minus" | "Multiply" | "Divide" => Ok(String::from(INT_CLASS_NAME)),

            // It's either '<', '=', or '<='
            _ => Ok(String::from(BOOL_CLASS_NAME)),
          }
        } else {
          Err(format!("For boolean or arithmetic operation, sub-expressions must be of type 'int' but found {left_expr_type} and {right_expr_type}"))
        }
      }

      Expression::Negate { expr } => {
        let negate_sub_expr = Self::get_symbol_type(*expr);
        match negate_sub_expr {
          Ok(sub_expr) if sub_expr == INT_CLASS_NAME => Ok(String::from(INT_CLASS_NAME)),
          Ok(sub_expr) => Err(format!("Negate sub-expression must be of type 'int' but found {sub_expr}")),
          Err(e) => Err(format!("Error in getting sub-expression type. Error {e}")),
        }
      }

      Expression::IsVoid { expr } => {
        let sub_expr_void = Self::get_symbol_type(*expr);
        match sub_expr_void {
          Ok(sub_expr) if sub_expr == VOID_CLASS_NAME => Ok(String::from(BOOL_CLASS_NAME)),
          Ok(sub_expr) => Err(format!("IsVoid sub-expression must be of type 'void' but found {sub_expr}")),
          Err(e) => Err(format!("Error in getting sub-expression type. Error {e}")),
        }
      }

      Expression::Not { expr } => {
        let sub_expr_type = Self::get_symbol_type(*expr);
        match sub_expr_type {
          Ok(sub_expr) if sub_expr == BOOL_CLASS_NAME => Ok(String::from(BOOL_CLASS_NAME)),
          Ok(sub_expr) => Err(format!("Not sub-expression must be of type 'bool' but found {sub_expr}")),
          Err(e) => Err(format!("Error in getting sub-expression type. Error {e}")),
        }
      }
      Expression::IntExpr { .. } => Ok(String::from(INT_CLASS_NAME)),
      Expression::BoolExpr { .. } => Ok(String::from(BOOL_CLASS_NAME)),
      Expression::StringExpr { .. } => Ok(String::from(STR_CLASS_NAME)),

      Expression::Block { .. } => Ok(String::from("Block")),

      Expression::Assign { name, expr } => Ok(String::from("Assign")),
      Expression::Dispatch { .. } => Ok(String::from("Dispatch")),
      Expression::Conditional { .. } => Ok(String::from("Conditional")),
      Expression::Loop { .. } => Ok(String::from("Loop")),
      Expression::Case { .. } => Ok(String::from("Case")),
      Expression::Let { .. } => Ok(String::from("Let")),

      Expression::New { type_name } => Ok(type_name.get_name()),
      Expression::IdentExpr { name } => Ok(name.get_name()),

      Expression::SelfTypeExpr { .. } => Ok(String::from("SelfTypeExpr")),
      Expression::SelfExpr => Ok(String::from("SelfExpr")),
    };

    expr_type
  }
}

impl From<Vec<HashMap<String, Symbol>>> for SymbolTable {
  fn from(value: Vec<HashMap<String, Symbol>>) -> Self {
    SymbolTable { symbols: value }
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct MethodTable {}

#[derive(PartialEq, Debug, Clone)]
pub struct ClassTable {}
