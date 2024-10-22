use std::fmt::{Display, Formatter};
use crate::models::Node;
use crate::models::symbols::Symbol;

#[derive(PartialEq, Debug, Clone)]
pub struct FormalNode {
    name: Symbol,
    f_type: Symbol,
}

impl Display for FormalNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


impl Node for FormalNode {}