use crate::tokens::Token;
use std::borrow::Cow;
use crate::expressions::case_expr::CaseBranch;

pub type Type = (Cow<'static, str>, u32, u32);
pub type Id = (Cow<'static, str>, u32, u32);

pub(crate) type LetInit = (Id, Type, Option<Box<Expression>>); // ID: TYPE [[ <- Expression ]]


impl From<Token> for Type { // Type and Symbol have same implementation of From
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
  feature_name: Id,
  formals: Option<Vec<Formal>>,
  return_type: Type,
  expr: Option<Box<Expression>>,
}

impl From<(Id, Option<Vec<Formal>>, Type, Box<Expression>)> for Feature {
  fn from((feature_name, formals, return_type, expr): (Id, Option<Vec<Formal>>, Type, Box<Expression>)) -> Self {
    Feature {
      feature_name,
      formals,
      return_type,
      expr: Some(expr),
    }
  }
}

impl From<(Id, Type, Box<Expression>)> for Feature {
  fn from((feature_name, return_type, expr): (Id, Type, Box<Expression>)) -> Self {
    Feature {
      feature_name,
      formals: None,
      return_type,
      expr: Some(expr),
    }
  }
}

impl From<(Id, Type)> for Feature {
  fn from((feature_name, return_type): (Id, Type)) -> Self {
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
  formal_name: Id,
  formal_type: Type,
}

impl From<(Id, Type)> for Formal {
  fn from((formal_name, formal_type): (Id, Type)) -> Self {
    Self {
      formal_name,
      formal_type,
    }
  }
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Expression {
  NoExpr,

  PartialAssign { expr: Box<Expression> },
  Assign { name: Id, expr: Box<Expression> },

  PartialDispatch { fn_name: Id, param_list: Vec<Expression> },
  PartialCastDispatch { cast_type: Type, fn_name: Id, param_list: Vec<Expression> },
  Dispatch {
    calling_expr: Box<Expression>, // If empty, then it's 
    cast_type_name: Option<Type>,
    fn_name: Id,
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

  PartialTypeOrSymbol { name_of_symbol_or_type: String, line_num: u32, line_pos: u32 },
  Ident { name: Id },

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
  
  pub(super) fn convert_to_dispatch(&self) -> Expression {
    if let Expression::PartialDispatch {fn_name, param_list} = self {
      Expression::Dispatch {
        fn_name: fn_name.clone(), 
        cast_type_name: None, 
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
      Expression::PartialTypeOrSymbol { .. } | 
      Expression::PartialBinary { .. }) 
  }

  pub(crate) fn get_type(&self) -> String {
    match self {
      Expression::NoExpr => String::from("NoExpr"),
      Expression::SelfExpr {..} => String::from("SelfExpr"),
      Expression::SelfTypeExpr {..} => String::from("SelfTypeExpr"),
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
      Expression::PartialTypeOrSymbol { .. } => String::from("PartialTypeOrSymbol"),
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
  /// - [Token::String]
  /// - [Token::Ident]
  /// - [Token::Int]
  /// - [Token::True]
  /// - [Token::False]
  /// - [Token::SelfType]
  fn from(token: Token) -> Self {
    match token {
      Token::Ident { value, line_num, line_pos } => 
        Expression::Ident { name: (Cow::from(value), line_num, line_pos) },

      Token::Int { value, line_num, line_pos } => Expression::Int { value, line_num, line_pos },
      Token::String { value, line_num, line_pos } => Expression::String { value, line_num, line_pos },
      
      Token::True { line_num, line_pos } => Expression::Bool { value: true, line_num, line_pos },
      Token::False { line_num, line_pos } => Expression::Bool { value: false, line_num, line_pos },
      
      Token::SelfType {line_num, line_pos} => Expression::SelfTypeExpr {line_num, line_pos},

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
