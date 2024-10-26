use crate::models::expression::ExpressionNode;

pub trait UnaryNode: ExpressionNode {}

pub struct NegateNode {
    expr: dyn ExpressionNode
}

pub struct NotNode {
    expr: dyn ExpressionNode
}

pub struct IsVoidNode {
    expr: dyn ExpressionNode
}
