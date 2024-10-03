use crate::tokens::Token;
use std::borrow::Cow;
use std::fmt::{Display, Formatter};

pub type Type = (Cow<'static, str>, u32, u32);
pub type Symbol = Type;
pub type CaseBranch = (Symbol, Type, Box<Expression>); // ID:TYPE => Expression 

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

  Assign { name: Symbol, expr: Box<Expression> },

  StaticDispatch { expr: Box<Expression>, type_name: Symbol, name: Symbol, actual: Box<Expression> },
  Dispatch { expr: Box<Expression>, name: Symbol, actual: Box<Expression> },

  Conditional { predicate: Box<Expression>, then_exp: Box<Expression>, else_exp: Box<Expression> },

  Loop { predicate: Box<Expression>, body: Symbol, actual: Box<Expression> },

  Case { switch_expression: Box<Expression>, branches: Vec<CaseBranch> },

  Block { expr_list: Option<Vec<Box<Expression>>> },

  Let { identifier: Symbol, type_declaration: Type, init: Box<Expression>, body: Box<Expression> },

  Plus { left: Box<Expression>, right: Box<Expression> },
  Minus { left: Box<Expression>, right: Box<Expression> },
  Multiply { left: Box<Expression>, right: Box<Expression> },
  Divide { left: Box<Expression>, right: Box<Expression> },

  Negate { expr: Box<Expression> },
  Not { expr: Box<Expression> },

  LessThan { left: Box<Expression>, right: Box<Expression> },
  Equal { left: Box<Expression>, right: Box<Expression> },
  LessThanOrEqual { left: Box<Expression>, right: Box<Expression> },

  Comp { expr: Box<Expression> },

  Ident { name: Symbol },

  Int { value: i32 },
  Bool { value: bool },
  String { value: String },

  New { type_name: Type },
  IsVoid { expr: Box<Expression> },

  Object { name: Symbol },
  
  Intermediate { expr: IntermediateExpression }
}

#[derive(PartialEq, Debug)]
enum IntermediateExpression {
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