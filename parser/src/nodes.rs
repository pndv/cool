struct Expr;

struct Id {
  name: String,
}

struct Type {
  name: String,
}

struct Assign {
  id: Id,
  expr: Expr,
}

struct Conditional {
  eval_expr: Expr,
  true_expr: Expr,
  false_expr: Expr,
}

struct Loop {
  eval_expr: Expr,
  exec_expr: Expr,
}

struct Block {
  expr_stmts: Vec<Expr>,
}
