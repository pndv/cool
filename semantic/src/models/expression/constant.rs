use crate::models::expression::ExpressionNode;

pub trait ConstantNode: ExpressionNode {}

pub struct IntNode {
    value: i32
}

pub struct StringNode {
    value: String
}

pub struct BoolNode {
    value: bool
}

