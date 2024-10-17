use crate::model::{Ident, Type};
use lexer::model::token::Token;

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

      _ => panic!("Non-constant token {:?}", token)
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

#[derive(PartialEq, Debug, Clone)]
pub struct LetInit {
  pub id: Ident,
  pub id_type: Type,
  pub expr: Option<Expression>,
}
