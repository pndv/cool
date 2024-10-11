use crate::class::Class;
use crate::tokens::{consume_required, gen_iter_till_token_or_end, is_eof, FilteredTokensIterator, SEMI_COLON_TYPE};
use crate::{class, tokens};
use class::gen_class;
use tokens::get_filtered_token_iter;

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Program {
  classes: Vec<Class>,
}

impl Default for Program {
  fn default() -> Self {
    Self::new()
  }
}

impl Program {
  #[must_use]
  pub fn new() -> Self {
    let classes: Vec<Class> = Vec::new();
    Program { classes }
  }

  pub(crate) fn add_class(&mut self, class: Class) {
    self.classes.push(class);
  }

  #[must_use]
  pub fn classes(&self) -> &Vec<Class> {
    &self.classes
  }
}

/// Program is a list of semicolon separated classes 
pub(crate) fn gen_program(token_iter: &mut FilteredTokensIterator) -> Program {
  let mut program: Program = Program::new();

  while !is_eof(token_iter) {
    let mut class_token_iter = gen_iter_till_token_or_end(token_iter, &SEMI_COLON_TYPE);
    let class: Class = gen_class(&mut class_token_iter);
    consume_required(token_iter, SEMI_COLON_TYPE);
    program.add_class(class);
  }

  program
}

#[must_use]
pub(crate) fn gen_programs(file_path: &str) -> Program {
  let mut token_iter: FilteredTokensIterator = get_filtered_token_iter(file_path);

  gen_program(&mut token_iter)
}

#[cfg(test)]
mod program_test {
  use crate::program::gen_programs;
  use std::fs::File;

  #[test]
  fn test_single_program() {
    let file = "test_resources/cool.cl";
    let program = gen_programs(file);
    println!("{:#?}", program);
  }

  #[test]
  fn test_large_program() {
    let file = "test_resources/arith.cl";
    let program = gen_programs(file);
    assert!(program.classes().len() > 1);
    assert_eq!(program.classes().len(), 7);
    println!("{:#?}", program.classes().len());
  }

  #[test]
  #[should_panic(expected = "Unexpected token: Dot { line_num: 5, line_pos: 54 }")]
  fn test_single_program_fail() {
    let f = File::open("test_resources/cool_bad.cl").unwrap();
    let file = "test_resources/cool_bad.cl";
    let program = gen_programs(file);
    println!("{:#?}", program);
  }
}
