use crate::generators::class::gen_class;
use crate::model::program::Program;
use lexer::iter::token::{BaseTokenIter, BufferedTokenIter, TokenIter};
use lexer::model::constants::SEMI_COLON_TYPE;
use std::fs::File;

#[must_use]
pub(crate) fn gen_programs(file_path: &str) -> Result<Program, String> {
  let err_message = format!("Cannot open file {}", &file_path);
  let file = File::open(file_path).expect(err_message.as_str());
  gen_program_from_file(file)
}

#[must_use]
pub(crate) fn gen_program_from_file(file: File) -> Result<Program, String> {
  let mut token_iter: TokenIter = TokenIter::from(file);
  gen_program(&mut token_iter)
}

/// Program is a list of semicolon separated classes 
pub(crate) fn gen_program(iter: &mut TokenIter) -> Result<Program, String> {
  let mut program: Program = Program::new();
  let mut errors = String::new();

  while iter.has_next() {
    let program_tokens = iter.collect_till(&SEMI_COLON_TYPE);
    
    if program_tokens.is_empty() {
      continue;  
    }
    
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
  use std::ffi::OsStr;
  use std::fs;
  use std::path::Path;

  #[test]
  fn test_single_program() {
    let file = File::open("../test_resources/programs/cool.cl").expect("Cannot open file");
    let program_result = gen_program_from_file(file);
    assert!(program_result.is_ok());
  }

  #[test]
  fn test_arith() {
    let file = File::open("../test_resources/programs/arith.cl").expect("Cannot open file");
    let program_result = gen_program_from_file(file);
    assert!(program_result.is_ok());
    let program = program_result.unwrap();
    assert!(program.classes().len() > 1);
    assert_eq!(program.classes().len(), 7);
    println!("{:#?}", program.classes().len());
  }

  #[test]
  fn test_lam() {
    let file = File::open("../test_resources/programs/lam.cl").expect("Cannot open file");
    let program_result = gen_program_from_file(file);
    assert!(program_result.is_ok());
    let program = program_result.unwrap();
    assert!(program.classes().len() > 1);
    assert_eq!(program.classes().len(), 11);
    println!("{:#?}", program.classes().len());
  }

  #[test]
  fn test_primes() {
    let file = File::open("../test_resources/programs/primes.cl").expect("Cannot open file");
    let program_result = gen_program_from_file(file);
    assert!(program_result.is_ok());
    let program = program_result.unwrap();
    assert_eq!(program.classes().len(), 1);
    println!("{:#?}", program.classes().len());
  }

  #[test]
  #[should_panic]
  fn test_single_program_fail() {
    let file = File::open("../../../test_resources/cool_bad.cl").expect("Cannot open file");
    let program = gen_program_from_file(file);
    assert!(program.is_err());
  }

  #[test]
  fn test_all_programs() {
    let filter_extn = OsStr::new("cl");
    let dir = Path::new("../test_resources/programs");
    let entries = fs::read_dir(dir).expect("Cannot open dir");

    for entry in entries {
      if entry.is_err() { continue; }
      let path = entry.unwrap().path();
      if path.extension() != Some(filter_extn) { continue; }
      print!("==== Processing: {:#?}", path);
      let file = File::open(path).expect("Cannot open file");
      let mut token_iter = TokenIter::from(file);
      let result = gen_program(&mut token_iter);
      println!(" = {:#?}", result.is_ok());
      assert!(result.is_ok());
    }

    println!("{:#?}", dir);
  }
}
