use std::fmt::{Debug, Display};

mod inheritance;
mod class;
mod symbols;
mod features;
mod expression;
mod formals;
// static mut TYPES: Vec<&str> = vec!["Int", "Bool" , "String","Object"];

trait Node: Display + PartialEq + Clone + Debug {}