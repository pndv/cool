use crate::models::expression::ExpressionNode;
use crate::models::Node;
use parser::model::expressions::Expression;
use std::fmt::{Debug, Display, Formatter};

pub struct AssignExprNode {
  
}

impl Display for AssignExprNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

impl Debug for AssignExprNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

impl From<Expression::Assign> for AssignExprNode {
  fn from(value: Expression::Assign) -> Self {
    match value {  Expression::Assign(name, expr) => {} }
  }
}

impl Node for AssignExprNode {
  fn get_file_name(&self) -> String {
    todo!()
  }

  fn get_line_number(&self) -> u32 {
    todo!()
  }

  fn get_column_number(&self) -> u32 {
    todo!()
  }

  fn get_symbol(&self, name: &str) -> Option<dyn Node> {
    todo!()
  }

  fn put_symbol(&mut self, name: &str, symbol: Box<dyn Node>) {
    todo!()
  }

  fn get_symbol_type(&self, name: &str) -> Option<String> {
    todo!()
  }

  fn decorate_ast(&mut self) {
    todo!()
  }
}

impl ExpressionNode for AssignExprNode {}
