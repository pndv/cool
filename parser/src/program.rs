use std::iter::Peekable;
use std::vec::IntoIter;
use crate::class::Class;
use lexer::tokens::{match_and_consume, gen_iter_till_token_or_end, is_eof, FilteredTokensIterator, SEMI_COLON_TYPE, Token};
use crate::{class, tokens};
use class::gen_class;
use lexer::iter::token::TokenIter;
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
pub(crate) fn gen_program(iter: &mut TokenIter) -> Program {
  let mut program: Program = Program::new();

  while iter.has_next() {
    let mut class_token_iter: Peekable<IntoIter<Token>> = iter.collect_till(&SEMI_COLON_TYPE);
    let class: Class = gen_class(&mut class_token_iter);
    assert!(iter.consume_required(&SEMI_COLON_TYPE));
    program.add_class(class);
  }
  
  program
}

#[must_use]
pub(crate) fn gen_programs(file_path: &str) -> Program {
  let mut token_iter: TokenIter = TokenIter::from(file_path.to_string());
  
  gen_program(&mut token_iter)
}

#[cfg(test)]
mod program_test {
  use crate::program::gen_programs;
  use std::ffi::OsStr;
  use std::fs;
  use std::fs::{DirEntry, File};
  use std::path::Path;

  #[test]
  fn test_single_program() {
    let file = "test_resources/programs/cool.cl";
    let program = gen_programs(file);
    println!("{:#?}", program);
  }

  #[test]
  fn test_large_program() {
    let file = "test_resources/programs/lam.cl";
    let program = gen_programs(file);
    assert!(program.classes().len() > 1);
    assert_eq!(program.classes().len(), 7);
    println!("{:#?}", program.classes().len());
  }

  #[test]
  fn test_programs() {
    let dir = Path::new("test_resources/programs");

    assert!(dir.exists());
    assert!(dir.is_dir());
    
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
      let f = match entry {
        Ok(dir_entry) if dir_entry.path().is_file() &&
            dir_entry.path().extension().and_then(OsStr::to_str) == Some("cl") => dir_entry.path(),
        Ok(_) => continue,
        Err(e) => {
          println!("Error {e}");
          continue;
        }
      };

      files.push(f);
    }

    for file in files {
      let file_path = file.to_str().unwrap();
      print!("> {file_path} ");

      let program = gen_programs(file_path);
      let class_count = program.classes().len();
      assert!(class_count > 0);

      println!("#Programs: {class_count}");
    }
  }

  #[test]
  #[should_panic(expected = "Unexpected token: Dot { line_num: 5, line_pos: 54 }")]
  fn test_single_program_fail() {
    let f = File::open("../test_resources/programs/cool_bad.cl_error").unwrap();
    let file = "test_resources/cool_bad.cl_error";
    let program = gen_programs(file);
    println!("{:#?}", program);
  }
}
