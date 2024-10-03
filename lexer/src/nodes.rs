pub type Symbol = String;
pub type Type = String;
pub type CaseBranch = (String, Symbol, Box<Expression>);

#[derive(PartialEq)]
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

#[derive(PartialEq)]
pub struct Class {
  type_name: Type,
  parent_type: Option<Type>, // if no parent is given, then 'Object' is the parent of all classes
  features: Option<Vec<Feature>>,
}

impl Class {
  const STR_OBJECT: &'static str = "Object";

  const OBJECT: Class = Class {
    type_name: Class::STR_OBJECT.to_string(),
    parent_type: None,
    features: None,
  };

  pub fn new(type_name: String, parent_type: Option<Type>, features: Option<Vec<Feature>>) -> Self {
    let parent: Type;
    if parent_type.is_some() {
      parent = parent_type.unwrap();
    } else {
      parent = Self::OBJECT.type_name.clone();
    }

    Class {
      type_name,
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

#[derive(PartialEq)]
pub struct Feature {
  ident_name: Symbol,
  formals: Option<Vec<Formal>>,
  return_type: Type,
  expr: Option<Box<Expression>>,
}

impl Feature {
  pub fn expr_attribute(ident_name: Symbol, return_type: Type, expr: Box<Expression>) -> Self {
    Feature {
      ident_name,
      formals: None,
      return_type,
      expr: Some(expr),
    }
  }

  pub fn simple_attribute(ident_name: Symbol, return_type: Type) -> Self {
    Feature {
      ident_name,
      formals: None,
      return_type,
      expr: None,
    }
  }

  pub fn method(ident_name: Symbol, formals: Option<Vec<Formal>>, return_type: Type, expr: Box<Expression>) -> Self {
    Feature {
      ident_name,
      formals,
      return_type,
      expr: Some(expr),
    }
  }
}

#[derive(PartialEq)]
pub struct Formal {
  ident_name: Symbol,
  ident_type: Type,
}

impl Formal {
  pub fn new(ident_name: String, ident_type: Type) -> Self {
    Self {
      ident_name,
      ident_type,
    }
  }
}

#[derive(PartialEq)]
pub enum Expression {
  NoExpr,

  Assign { name: Symbol, expr: Box<Expression> },

  StaticDispatch { expr: Box<Expression>, type_name: Symbol, name: Symbol, actual: Box<Expression> },
  Dispatch { expr: Box<Expression>, name: Symbol, actual: Box<Expression> },

  Conditional { predicate: Box<Expression>, then_exp: Box<Expression>, else_exp: Box<Expression> },

  Loop { predicate: Box<Expression>, body: Symbol, actual: Box<Expression> },

  Case { switch_expression: Box<Expression>, branches: Vec<CaseBranch> },

  Block { body: Box<Expression> },

  Let { identifier: Symbol, type_declaration: Symbol, init: Box<Expression>, body: Box<Expression> },

  Plus { left: Box<Expression>, right: Box<Expression> },
  Minus { left: Box<Expression>, right: Box<Expression> },
  Multiply { left: Box<Expression>, right: Box<Expression> },
  Divide { left: Box<Expression>, right: Box<Expression> },

  Negate { expr: Box<Expression> },

  LessThan { left: Box<Expression>, right: Box<Expression> },
  Equal { left: Box<Expression>, right: Box<Expression> },
  LessThanOrEqual { left: Box<Expression>, right: Box<Expression> },

  Comp { expr: Box<Expression> },

  Int { value: i32 },
  Bool { value: bool },
  String { value: String },

  New { type_name: Symbol },
  IsVoid { expr: Box<Expression> },

  Object { name: Symbol },
}
