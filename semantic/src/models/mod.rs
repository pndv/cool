use std::fmt::{Debug, Display};

pub mod class;
pub mod symbols;
pub mod features;
pub mod expression;
pub mod formals;
pub mod program;

pub trait Node: Display + Debug {
  fn get_file_name(&self) -> String {"".to_string()} // Fill with the corresponding file name in the child implementations
  fn get_line_number(&self) -> u32 {0}
  fn get_column_number(&self) -> u32 {0}
  
}
