use crate::models::expression::ExpressionNode;
use crate::models::Node;

pub trait Binary: ExpressionNode {}

pub struct AddExpression {
    left: dyn ExpressionNode,
    right: dyn ExpressionNode,
}

pub struct SubExpression {
    left: dyn ExpressionNode,
    right: dyn ExpressionNode,
}

pub struct MulExpression {
    left: dyn ExpressionNode,
    right: dyn ExpressionNode,
}

pub struct DivExpression {
    left: dyn ExpressionNode,
    right: dyn ExpressionNode,
}
