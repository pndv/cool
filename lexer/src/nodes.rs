use crate::tokens::Token;
use std::borrow::Cow;

pub type Type = (Cow<'static, str>, u32, u32);
pub type Symbol = Type;
pub(crate) type CaseBranch = (Symbol, Type, Box<Expression>); // ID:TYPE => Expression 
pub(crate) type LetInit = (Symbol, Type, Option<Box<Expression>>); // ID: TYPE [[ <- Expression ]]

impl From<Token> for Type {
  fn from(value: Token) -> Self {
    match value {
      Token::Ident { value, line_num, line_pos } => (Cow::Owned(value), line_num, line_pos),
      _ => panic!("Unexpected token {:?}", value),
    }
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Program {
  classes: Vec<Class>,
}

impl Default for Program {
  fn default() -> Self {
    Self::new()
  }
}

impl Program {
  #[must_use]
  pub fn new() -> Self {
    let classes: Vec<Class> = Vec::new();
    Program { classes }
  }

  pub fn add_class(&mut self, class: Class) {
    self.classes.push(class);
  }

  pub fn classes(&self) -> &Vec<Class> {
    &self.classes
  }
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Class {
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
  pub(crate) fn new(class_type: Type, parent_type: Option<Type>, features: Option<Vec<Feature>>) -> Self {
    let parent: Type = if parent_type.is_some() {
      parent_type.unwrap()
    } else {
      OBJECT.class_type.clone()
    };

    Class {
      class_type,
      parent_type: Some(parent),
      features,
    }
  }

  pub fn add_feature(&mut self, feature: Feature) {
    if self.features.is_none() {
      self.features = Some(Vec::new());
    }

    if let Some(ref mut features) = self.features {
      features.push(feature);
    }
  }
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Feature {
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

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Formal {
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

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Expression {
  NoExpr,
  SelfExpr,

  PartialAssign { expr: Box<Expression> },
  Assign { name: Symbol, expr: Box<Expression> },

  PartialDispatch { fn_name: Symbol, param_list: Vec<Expression> },
  PartialCastDispatch { cast_type: Type, partial_dispatch: Box<Expression> },
  Dispatch {
    calling_expr: Box<Expression>,
    cast_type_name: Option<Type>,
    fn_name: Symbol,
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

  Ident { name: Symbol },

  Int { value: i32, line_num: u32, line_pos: u32 },
  Bool { value: bool, line_num: u32, line_pos: u32 },
  String { value: String, line_num: u32, line_pos: u32 },

  New { type_name: Type },
  IsVoid { expr: Box<Expression> },

  // Object { name: Symbol },
}

impl Expression {
  pub fn is_partial(&self) -> bool {
    matches!(self, Expression::PartialDispatch { .. } | 
      Expression::PartialCastDispatch { .. } | 
      Expression::PartialBinary { .. })
  }

  pub(crate) fn get_type(&self) -> String {
    match self {
      Expression::NoExpr => String::from("NoExpr"),
      Expression::SelfExpr => String::from("SelfExpr"),
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

#[cfg(test)]
mod test {
  use crate::nodes::Expression;

  #[test]
  fn test_get_type() {
    assert_eq!(Expression::Int { value: 10, line_num: 5, line_pos: 10 }.get_type(), "Int");
  }
}
