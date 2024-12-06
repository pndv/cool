use crate::models::Node;
use parser::model::expressions::Expression;

#[derive(Debug, PartialEq, Clone)]
pub enum  ExpressionNode {
    Add{ left: Box<ExpressionNode>, right: Box<ExpressionNode> },
    Sub{ left: Box<ExpressionNode>, right: Box<ExpressionNode> },
    Mul{ left: Box<ExpressionNode>, right: Box<ExpressionNode> },
    Div{ left: Box<ExpressionNode>, right: Box<ExpressionNode> },
    Equal{ left: Box<ExpressionNode>, right: Box<ExpressionNode> },
    Not{ expr: Box<ExpressionNode> },
    Negate{ expr: Box<ExpressionNode> },

    Int {val: i32},
    Str {val: String},
    Bool {val: bool},
}

impl Node for Expression {}

impl From<Expression> for ExpressionNode {
    fn from(value: Expression) -> Self {
        todo!()
    }
}

impl From<Box<Expression>> for ExpressionNode {
    fn from(value: Box<Expression>) -> Self {
        todo!()
    }
}