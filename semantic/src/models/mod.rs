use std::fmt::{Debug, Display};

pub mod class;
pub mod symbols;
pub mod features;
pub mod expression;
pub mod formals;
pub mod program;

pub trait Node: Display + Debug {}
