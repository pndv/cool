﻿trait TreeNode {
  // fn dump(&self);
  fn get_line_number(&self) -> u32;
  fn get_children(&self) -> Option<Vec<Box<dyn TreeNode>>>;
  fn set_line_number(&mut self, line_number: u32);
}



type Symbol = String;
type Type = String;

struct Program {
  classes: Vec<Class>,
}

struct Class {
  type_name: Type,
  parent_type: Type, // if no parent is given, then 'Object' is the parent of all classes
  features: Option<Vec<Feature>>,
}


struct Feature {
  ident: Symbol,
  formals: Option<Vec<Formal>>,
  feature_type: Type,
  expr: Box<dyn Expr<Type>>
}

struct Formal {
  ident: Symbol,
  formal_type: Type,
}


enum Expression {
  Assign { name: Symbol, expr: Box<Expression>},
  
  StaticDispatch {expr: Box<Expression>, type_name: Symbol, name: Symbol, actual: Box<Expression>},
  Dispatch {expr: Box<Expression>, name: Symbol, actual: Box<Expression>},
  
  Conditional {predicate: Box<Expression>, then_exp: Box<Expression>, else_exp: Box<Expression>},
  
  Loop {predicate: Box<Expression>, body: Symbol, actual: Box<Expression>},
  
  CaseBranch {name: Symbol, type_declaration: Symbol, expr: Box<Expression>},
  Case {}
}

struct IdentNode {
  ident_name: String,
  line_number: u32,
}

struct TrueNode {
  line_number: u32,
}

struct FalseNode {
  line_number: u32,
}

struct IntNode {
  value: i32,
  line_number: u32,
}

struct StringNode {
  value: String,
  line_number: u32,
}



trait Expr<T> {
  fn eval(&self) -> T;
}

struct TrueExpr;
impl Expr<bool> for TrueExpr {
  fn eval(&self) -> bool {
    true
  }
}


struct FalseExpr;
impl Expr<bool> for FalseExpr {
  fn eval(&self) -> bool {
    false
  }
}


struct StringExpr {
  value: String
}

impl StringExpr {
  fn set_value(&mut self, value: String) {
    assert!(value.chars().count() <= 1024,
            "String exceeds maximum length of 1024 characters; has {} characters", value.len());

    self.value = value;
  }
}

impl Expr<String> for StringExpr {
  fn eval(&self) -> String {
    self.value.clone()
  }
}

struct IntExpr {
  value: i32
}

impl IntExpr {
  fn set_value(&mut self, value: i32) {
    self.value = value;
  }
}

impl Expr<i32> for IntExpr {
  fn eval(&self) -> i32 {
    self.value
  }
}

struct IdentExpr {
  id: Symbol
}

impl Expr<Symbol> for IdentExpr {
  fn eval(&self) -> Symbol {
    self.id.clone()
  }
}

struct NotExpr<T> {
  expr: dyn Expr<T>
}

struct EqualsExpr<T, U> {
  left_expr: Box<dyn Expr<T>>,
  right_expr: Box<dyn Expr<U>>,
}


struct IsVoidExpr {
  expr: dyn Expr<bool>
}

enum ExprType {
  Assign, 
  
  Conditional,
  Loop,
  Block,
  
  Case,
  CaseBranch,
  
  StaticDispatch,
  Dispatch,
  
  Let,
  
  Plus,
  Subtract,
  Multiply,
  Divide,
  Negate,
  
  LessThan,
  Equals,
  LessThanEquals,
  
  Not,
  
  Int,
  Bool,
  String,
  
  IsVoid,
  NoExpr,
  
  Object
}