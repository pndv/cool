use crate::nodes::Expression::{PartialBinary, PartialDispatch};
use crate::tokens::Token;
use std::borrow::Cow;
use std::fmt::Display;

pub type Type = (Cow<'static, str>, u32, u32);
pub type Symbol = Type;
pub type CaseBranch = (Symbol, Type, Box<Expression>); // ID:TYPE => Expression 
pub type LetInit = (Symbol, Type, Option<Box<Expression>>); // ID: TYPE [[ <- Expression ]]

impl From<Token> for Type {
  fn from(value: Token) -> Self {
    match value {
      Token::Ident { value, line_num, line_pos } => (Cow::Owned(value), line_num, line_pos),
      _ => panic!("Unexpected token {:?}", value),
    }
  }
}

#[derive(PartialEq, Debug)]
pub struct Program {
  classes: Vec<Class>,
}

impl Program {
  pub fn new() -> Self {
    let classes: Vec<Class> = Vec::new();
    Program { classes }
  }

  pub fn add_class(&mut self, class: Class) {
    self.classes.push(class);
  }
}

#[derive(PartialEq, Debug)]
pub struct Class {
  class_type: Type,
  parent_type: Option<Type>, // if no parent is given, then 'Object' is the parent of all classes
  features: Option<Vec<Feature>>,
}

const OBJECT: Class = Class {
  class_type: (Cow::Borrowed("Object"), 0, 0),
  parent_type: None,
  features: None,
};

impl Class {
  pub fn new(class_type: Type, parent_type: Option<Type>, features: Option<Vec<Feature>>) -> Self {
    let parent: Type;
    if parent_type.is_some() {
      parent = parent_type.unwrap();
    } else {
      parent = OBJECT.class_type.clone();
    }

    Class {
      class_type,
      parent_type: Some(parent),
      features,
    }
  }

  pub fn add_feature(&mut self, feature: Feature) {
    if self.features == None {
      self.features = Some(Vec::new());
    }

    if let Some(ref mut features) = self.features {
      features.push(feature);
    }
  }
}

#[derive(PartialEq, Debug)]
pub struct Feature {
  feature_name: Symbol,
  formals: Option<Vec<Formal>>,
  return_type: Type,
  expr: Option<Box<Expression>>,
}

impl From<(Symbol, Option<Vec<Formal>>, Type, Box<Expression>)> for Feature {
  fn from((feature_name, formals, return_type, expr): (Symbol, Option<Vec<Formal>>, Type, Box<Expression>)) -> Self {
    Feature {
      feature_name,
      formals,
      return_type,
      expr: Some(expr),
    }
  }
}

impl From<(Symbol, Type, Box<Expression>)> for Feature {
  fn from((feature_name, return_type, expr): (Symbol, Type, Box<Expression>)) -> Self {
    Feature {
      feature_name,
      formals: None,
      return_type,
      expr: Some(expr),
    }
  }
}

impl From<(Symbol, Type)> for Feature {
  fn from((feature_name, return_type): (Symbol, Type)) -> Self {
    Feature {
      feature_name,
      formals: None,
      return_type,
      expr: None,
    }
  }
}

#[derive(PartialEq, Debug)]
pub struct Formal {
  formal_name: Symbol,
  formal_type: Type,
}

impl From<(Symbol, Type)> for Formal {
  fn from((formal_name, formal_type): (Symbol, Type)) -> Self {
    Self {
      formal_name,
      formal_type,
    }
  }
}

#[derive(PartialEq, Debug)]
pub enum Expression {
  NoExpr,
  SelfExpr,

  Assign { name: Symbol, expr: Box<Expression> },

  PartialDispatch { fn_name: Symbol, param_list: Vec<Box<Expression>> },
  PartialCastDispatch { cast_type: Type, partial_dispatch: Box<Expression> },
  Dispatch {
    calling_expr: Box<Expression>,
    cast_type_name: Option<Type>,
    fn_name: Symbol,
    param_list: Vec<Box<Expression>>, // if no parameters, then it's a single list of [NoExpr] 
  },

  Conditional { predicate: Box<Expression>, then_expr: Box<Expression>, else_expr: Box<Expression> },

  Loop { predicate: Box<Expression>, body: Box<Expression> },

  Case { switch_expression: Box<Expression>, branches: Vec<CaseBranch> },

  Block { expr_list: Vec<Box<Expression>> }, // must have at least one `Expression` in the list

  Let { let_init: Vec<LetInit>, in_expr: Box<Expression> },

  PartialBinary { binary_token: Token, right_expr: Box<Expression> }, // for constructing binary expressions
  Plus { left: Box<Expression>, right: Box<Expression> },
  Minus { left: Box<Expression>, right: Box<Expression> },
  Multiply { left: Box<Expression>, right: Box<Expression> },
  Divide { left: Box<Expression>, right: Box<Expression> },
  LessThan { left: Box<Expression>, right: Box<Expression> },
  Equal { left: Box<Expression>, right: Box<Expression> },
  LessThanOrEqual { left: Box<Expression>, right: Box<Expression> },

  Comp { expr: Box<Expression> },
  Negate { expr: Box<Expression> },

  Not { expr: Box<Expression> },

  Ident { name: Symbol },

  Int { value: i32, line_num: u32, line_pos: u32 },
  Bool { value: bool, line_num: u32, line_pos: u32 },
  String { value: String, line_num: u32, line_pos: u32 },

  New { type_name: Type },
  IsVoid { expr: Box<Expression> },

  Object { name: Symbol },

}

impl Expression {
  fn is_partial(&self) -> bool {
    match self {
      PartialDispatch { .. } | Expression::PartialCastDispatch { .. } | PartialBinary { .. } => true,
      _ => false,
    }
  }
}

impl From<Token> for Expression {
  /// Only for 
  /// - [Token::Str]
  /// - [Token::Ident]
  /// - [Token::Int]
  /// - [Token::True]
  /// - [Token::False]
  fn from(token: Token) -> Self {
    match token {
      Token::Str { value, line_num, line_pos } =>
        Expression::String { value, line_num, line_pos },

      Token::Ident { .. } =>
        Expression::Ident { name: Symbol::from(token) },

      Token::Int { value, line_num, line_pos } => Expression::Int { value, line_num, line_pos },
      Token::True { line_num, line_pos } => Expression::Bool { value: true, line_num, line_pos },
      Token::False { line_num, line_pos } => Expression::Bool { value: false, line_num, line_pos },

      _ => panic!("Non-constant token {:?}", token)
    }
  }
}

#[derive(PartialEq, Debug)]
pub(crate) enum ReadState {
  ExpressionStart,

  IdentStarting,
  LetIn,

  CaseOf,
  CaseEnd,

  WhileLoop,
  WhileEnd,

  ConditionalThen,
  ConditionalElse,
  ConditionalEnd,

  BinaryPlus,
  BinaryMinus,
  BinaryMultiply,
  BinaryDivide,
  BinaryLessThan,
  BinaryLessThanOrEqual,
  BinaryEqual,
}