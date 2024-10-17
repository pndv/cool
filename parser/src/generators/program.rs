use crate::generators::class::gen_class;
use crate::model::program::Program;
use lex::iter::token::{BaseTokenIter, BufferedTokenIter, TokenIter};
use lex::model::constants::SEMI_COLON_TYPE;
use std::fs::File;

#[must_use]
pub(crate) fn gen_programs(file_path: &str) -> Result<Program, String> {
  let file = File::open(file_path).expect(format!("Cannot open file {file_path}").as_str());
  gen_program_from_file(file)
}

#[must_use]
pub(crate) fn gen_program_from_file(file:File) -> Result<Program, String> {
  let mut token_iter: TokenIter = TokenIter::from(file);
  gen_program(&mut token_iter)
}

/// Program is a list of semicolon separated classes 
pub(crate) fn gen_program(iter: &mut TokenIter) -> Result<Program, String> {
  let mut program: Program = Program::new();
  let mut errors = String::new();

  while iter.has_next() {
    let program_tokens = iter.collect_till(&SEMI_COLON_TYPE);
    let mut buffered_iter = BufferedTokenIter::from(program_tokens);

    match gen_class(&mut buffered_iter) {
      Ok(class) => program.add_class(class),
      Err(e) => errors.push_str(&e),
    }

    iter.consume_required(&SEMI_COLON_TYPE)?;
  }

  if errors.is_empty() {
    Ok(program)
  } else {
    Err(errors)
  }
}

#[cfg(test)]
mod program_test {
  use super::*;

  #[test]
  fn test_single_program() {
    let file = File::open("../test_resources/programs/cool.cl").expect("Cannot open file");
    let program_result = gen_program_from_file(file);
    assert!(program_result.is_ok());
  }

  #[test]
  fn test_large_program() {
    let file = File::open("../test_resources/programs/arith.cl").expect("Cannot open file");
    let program_result = gen_program_from_file(file);
    assert!(program_result.is_ok());
    let program = program_result.unwrap();
    assert!(program.classes().len() > 1);
    assert_eq!(program.classes().len(), 7);
    println!("{:#?}", program.classes().len());
  }

  #[test]
  #[should_panic]
  fn test_single_program_fail() {
    let file = File::open("../test_resources/programs/cool_bad.cl").expect("Cannot open file");
    let program = gen_program_from_file(file);
    assert!(program.is_err());
  }
}
