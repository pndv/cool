use std::fmt::{Debug, Display};

pub mod class;
pub mod symbols;
pub mod features;
pub mod expression;
pub mod formals;
pub mod program;

pub trait Node: Display + Debug {
  fn get_file_name(&self) -> String; // Fill with the corresponding file name in the child implementations
  fn get_line_number(&self) -> u32;
  fn get_column_number(&self) -> u32;
  fn get_symbol(&self, name: &str) -> Option<dyn Node>;
  fn put_symbol(&mut self, name: &str, symbol: Box<dyn Node>);
  fn get_symbol_type(&self, name: &str) -> Option<String>;
  fn get_parent(&self) -> Option<Box<dyn Node>>;
  fn decorate_ast(&mut self);
}
