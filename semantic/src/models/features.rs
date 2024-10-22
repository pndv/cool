use crate::models::expression::ExpressionNode;
use crate::models::formals::FormalNode;
use crate::models::Node;

pub trait FeatureNode: Node {
    fn get_formals() -> Vec<FormalNode>;
    fn get_expressions() -> Vec<dyn ExpressionNode>;
}

#[derive(Debug, PartialEq, Clone)]
pub struct AttributeNode {

}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodNode {

}