trait TreeNode {
  // fn dump(&self);
  fn get_line_number(&self) -> u32;
  fn get_children(&self) -> Option<Vec<Box<dyn TreeNode>>>;
}

type Ident = String;
type Type = String;


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
  id: Ident
}

impl Expr<Ident> for IdentExpr {
  fn eval(&self) -> Ident {
    self.id.clone()
  }
}

struct NotExpr {
  expr: dyn Expr<bool>
}

struct IsVoidExpr {
  expr: dyn Expr<bool>
}

struct Formal {
  ident: Ident,
  formal_type: Type,
}

struct Feature {
  ident: Ident,
  formals: Option<Vec<Formal>>,
  feature_type: Type,
  expr: dyn Expr<Type>
}

struct Class {
  type_name: Type,
  parent_type: Type, // if no parent is given, then 'Object' is the parent of all classes
  features: Option<Vec<Feature>>,
}