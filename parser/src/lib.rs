use crate::generators::program::gen_program_from_file;
use crate::model::program::Program;
use std::fs::File;

pub(crate) mod generators;
pub mod model;

pub fn get_ast_from_file_path(file_path: &str) -> Result<Program, String> {
    let f = File::open(file_path).expect("Unable to open file");
    get_ast(f)
}
pub fn get_ast(file: File) -> Result<Program, String> {
    gen_program_from_file(file)
}

#[cfg(test)]
pub(crate) mod test {
  use lexer::iter::token::{BufferedTokenIter, TokenIter};
  use std::fs::File;

  pub(crate) fn get_buffered_iter(file: File) -> BufferedTokenIter {
        let token_iter = TokenIter::from(file);
        BufferedTokenIter::from(token_iter)
    }
}
