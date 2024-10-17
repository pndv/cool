use crate::model::{Ident, Type};
use lexer::model::token::Token;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Expression {
  PartialAssign { expr: Box<Expression> },
  Assign { name: Ident, expr: Box<Expression> },

  PartialDispatch { fn_name: Ident, param_list: Vec<Expression> },
  PartialCastDispatch { cast_type: Option<Type>, fn_name: Ident, param_list: Vec<Expression> },
  Dispatch {
    calling_expr: Box<Expression>, // If empty, then it's 
    cast_type: Option<Type>,
    fn_name: Ident,
    param_list: Vec<Expression>, // if no parameters, then it's a single list of [NoExpr] 
  },

  Conditional { predicate: Box<Expression>, then_expr: Box<Expression>, else_expr: Box<Expression> },

  Loop { predicate: Box<Expression>, body: Box<Expression> },

  Case { switch_expression: Box<Expression>, branches: Vec<CaseBranch> },

  Block { expr_list: Vec<Expression> }, // must have at least one `Expression` in the list

  Let { let_init: Vec<LetInit>, in_expr: Box<Expression> },

  PartialBinary { binary_token: Token, right_expr: Box<Expression> }, // for constructing binary expressions
  Plus { left: Box<Expression>, right: Box<Expression> },
  Minus { left: Box<Expression>, right: Box<Expression> },
  Multiply { left: Box<Expression>, right: Box<Expression> },
  Divide { left: Box<Expression>, right: Box<Expression> },
  LessThan { left: Box<Expression>, right: Box<Expression> },
  Equal { left: Box<Expression>, right: Box<Expression> },
  LessThanOrEqual { left: Box<Expression>, right: Box<Expression> },

  Negate { expr: Box<Expression> },

  Not { expr: Box<Expression> },

  Ident { name: Ident },

  Int { value: i32, line_num: u32, line_pos: u32 },
  Bool { value: bool, line_num: u32, line_pos: u32 },
  String { value: String, line_num: u32, line_pos: u32 },

  SelfTypeExpr { line_num: u32, line_pos: u32 },
  SelfExpr,

  New { type_name: Type },
  IsVoid { expr: Box<Expression> },

  // Object { name: Symbol },
}

impl Expression {
  pub(crate) fn convert_to_dispatch(&self) -> Expression {
    if let Expression::PartialDispatch { fn_name, param_list } = self {
      Expression::Dispatch {
        fn_name: fn_name.clone(),
        cast_type: None,
        calling_expr: Box::from(Expression::SelfExpr),
        param_list: param_list.clone(),
      }
    } else {
      panic!("Can only convert PartialDispatch to Dispatch");
    }
  }

  pub fn is_partial(&self) -> bool {
    matches!(self, Expression::PartialDispatch { .. } | 
      Expression::PartialCastDispatch { .. } | 
      Expression::PartialAssign { .. } | 
      Expression::PartialBinary { .. })
  }

  pub(crate) fn get_type(&self) -> String {
    match self {
      // Expression::NoExpr => String::from("NoExpr"),
      Expression::SelfExpr { .. } => String::from("Self"),
      Expression::SelfTypeExpr { .. } => String::from("SelfType"),
      Expression::PartialAssign { .. } => String::from("PartialAssign"),
      Expression::Assign { .. } => String::from("Assign"),
      Expression::PartialDispatch { .. } => String::from("PartialDispatch"),
      Expression::PartialCastDispatch { .. } => String::from("PartialCastDispatch"),
      Expression::Dispatch { .. } => String::from("Dispatch"),
      Expression::Conditional { .. } => String::from("Conditional"),
      Expression::Loop { .. } => String::from("Loop"),
      Expression::Case { .. } => String::from("Case"),
      Expression::Block { .. } => String::from("Block"),
      Expression::Let { .. } => String::from("Let"),
      Expression::PartialBinary { .. } => String::from("PartialBinary"),
      Expression::Plus { .. } => String::from("Plus"),
      Expression::Minus { .. } => String::from("Minus"),
      Expression::Multiply { .. } => String::from("Multiply"),
      Expression::Divide { .. } => String::from("Divide"),
      Expression::LessThan { .. } => String::from("LessThan"),
      Expression::Equal { .. } => String::from("Equal"),
      Expression::LessThanOrEqual { .. } => String::from("LessThanOrEqual"),
      Expression::Negate { .. } => String::from("Negate"),
      Expression::Not { .. } => String::from("Not"),
      Expression::Ident { .. } => String::from("Ident"),
      Expression::Int { .. } => String::from("Int"),
      Expression::Bool { .. } => String::from("Bool"),
      Expression::String { .. } => String::from("String"),
      Expression::New { .. } => String::from("New"),
      Expression::IsVoid { .. } => String::from("IsVoid"),
    }
  }
}

impl From<Token> for Expression {
  /// Only for 
  /// - [`Token::String`]
  /// - [`Token::Ident`]
  /// - [`Token::Int`]
  /// - [`Token::True`]
  /// - [`Token::False`]
  /// - [`Token::SelfType`]
  fn from(token: Token) -> Self {
    match token {
      Token::Ident { value, .. } => Expression::Ident { name: Ident::from(value) },

      Token::Int { value, line_num, line_pos } => Expression::Int { value, line_num, line_pos },
      Token::String { value, line_num, line_pos } => Expression::String { value, line_num, line_pos },

      Token::True { line_num, line_pos } => Expression::Bool { value: true, line_num, line_pos },
      Token::False { line_num, line_pos } => Expression::Bool { value: false, line_num, line_pos },

      Token::SelfType { line_num, line_pos } => Expression::SelfTypeExpr { line_num, line_pos },

      _ => panic!("Non-constant token {token}")
    }
  }
}

impl Display for Expression {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Expression::PartialAssign { expr } => write!(f, "Partial '<-' [\n \t{} \n]", expr),
      Expression::PartialDispatch { fn_name, param_list } => {
        let mut str = format!("Partial Dispatch {fn_name} (");
        for param in param_list {
          str.push_str(&format!(" {} ", param));
        }
        str.push_str(")");
        write!(f, "{fn_name} ({str})")
      }
      Expression::PartialCastDispatch { fn_name, cast_type, param_list } => {
        match cast_type {
          None => write!(f, "Partial Dispatch {fn_name} ({param_list:?})"),
          Some(cast) => {
            let mut param_str = String::new();
            for param in param_list {
              param_str.push_str(&format!(" {param} "));
            }

            write!(f, "Partial Dispatch {fn_name} AS {cast} ({param_str})")
          }
        }
      }
      Expression::Assign { name, expr } => write!(f, "{} '<-' [\n \t{} \n]", name, expr),

      Expression::Dispatch { calling_expr, fn_name, cast_type, param_list, } => {
        let mut param_str = String::new();
        for param in param_list {
          param_str.push_str(&format!(" {} ", param));
        }

        match cast_type {
          None => write!(f, "DISPATCH\n\tFROM: {} \n\t\tCALL: {fn_name} \n\t\t\tPARAMS: {param_str}", calling_expr),
          Some(cast) => write!(f, "DISPATCH\n\tFROM: {} \n\t\tCALL: {fn_name} AS {cast} \n\t\t\tPARAMS: {param_str}", calling_expr),
        }
      }
      
      Expression::Conditional { predicate, then_expr, else_expr } => { 
        write!(f, "If \t {} \n\tThen: [ {} ] \n\tElse: [ {} ]", predicate, then_expr, else_expr) 
      },

      Expression::Loop { predicate, body } => write!(f, "While {} \n\t [ {} \n\t ]", predicate, body),

      Expression::Case { switch_expression, branches } => {
        let mut str_branch = String::new();
        for branch in branches {
          str_branch.push_str(&format!("\t=> {}\n ", branch));
        }
        
        write!(f, "CASE {}\n\tBranches:\n{str_branch}", switch_expression)
      }

      Expression::Block { expr_list } => {
        let mut str_expr = String::new();
        for expr in expr_list {
          str_expr.push_str(&format!("\t{} \n", expr));
        }
        
        write!(f, "BLOCK:[ \t{str_expr} \n]") 
      },

      Expression::Let { let_init, in_expr } => { 
        let mut str_let_init = String::new();
        for expr in let_init {
          str_let_init.push_str(&format!("{} ", expr));
        }
        
        write!(f, "LET\n\tInitialisations:\n\t{str_let_init}\n\tIn:\n\t{}", in_expr) 
      }

      Expression::PartialBinary { binary_token, right_expr } => write!(f, "Partial {binary_token} [ {} ]", right_expr),

      Expression::Plus { left, right } => write!(f, "[ {} ] + [ {} ]", left, right),
      Expression::Minus { left, right } => write!(f, "[ {} ] - [ {} ]", left, right),
      Expression::Multiply { left, right } => write!(f, "[ {} ] * [ {} ]", left, right),
      Expression::Divide { left, right } => write!(f, "[ {} ] / [ {} ]", left, right),
      Expression::LessThan { left, right } => write!(f, "[ {} ] < [ {} ]", left, right),
      Expression::Equal { left, right } => write!(f, "[ {} ] = [ {} ]", left, right),
      Expression::LessThanOrEqual { left, right } => write!(f, "[ {} ] <= [ {} ]", left, right),

      Expression::Negate { expr } => write!(f, "~ [ {} ]", expr),
      Expression::Not { expr } => write!(f, "not [ {} ]", expr),
      Expression::IsVoid { expr } => write!(f, "is-void [ {} ]", expr),

      Expression::New { type_name } => write!(f, "new [ {type_name} ]"),
      Expression::Ident { name } => write!(f, "Identifier [ {name} ]"),

      Expression::Int { value, .. } => write!(f, "Int [ {value} ]"),
      Expression::Bool { value, .. } => write!(f, "Bool [ {value} ]"),
      Expression::String { value, .. } => write!(f, "String [ \"{value}\" ]"),

      Expression::SelfTypeExpr { .. } => write!(f, "SelfType"),
      Expression::SelfExpr => write!(f, "Self"),
    }
  }
}

// ID:TYPE => Expression
#[derive(PartialEq, Debug, Clone)]
pub struct CaseBranch {
  pub id: Ident,
  pub id_type: Type,
  pub expr: Expression,
}

impl Display for CaseBranch {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Case: {} : ({}) => {}", self.id, self.id_type, self.expr)
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct LetInit {
  pub id: Ident,
  pub id_type: Type,
  pub expr: Option<Expression>,
}

impl Display for LetInit {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match &self.expr {
      None => write!(f, "LET: {}: {} ", self.id, self.id_type),
      Some(e) => write!(f, "LET: {}: {} = {}", self.id, self.id_type, e)
    }
  }
}
