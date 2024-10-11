pub mod constants;

trait Expr {}

struct Id {
  name: String,
}

struct Type {
  name: String,
}

struct Assign {
  id: Id,
  expr: dyn Expr,
}

struct Conditional {
  eval_expr: Box<dyn Expr>,
  true_expr: Box<dyn Expr>,
  false_expr: Box<dyn Expr>,
}

struct Loop {
  eval_expr: Box<dyn Expr>,
  exec_expr: Box<dyn Expr>,
}

struct Block {
  expr_stmts: Vec<Box<dyn Expr>>,
}

struct Let {}

struct Dispatch {}
