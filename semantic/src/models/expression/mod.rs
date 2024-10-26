pub mod binary;
mod unary;
mod constant;

use crate::models::Node;

pub trait ExpressionNode: Node {}