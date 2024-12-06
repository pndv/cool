use crate::models::symbols::Symbol;
use crate::models::Node;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug, Clone)]
pub struct FormalNode {
    name: Symbol,
    f_type: Symbol,
}

impl Display for FormalNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FORMAL | Name: {}, Type: {}", self.name.name, self.f_type.name)
    }
}


impl Node for FormalNode {}