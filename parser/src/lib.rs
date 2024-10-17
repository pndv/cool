pub mod model;
pub(crate) mod generators;

#[cfg(test)]
pub(crate) mod test {
  use lexer::iter::token::{BufferedTokenIter, TokenIter};
  use std::fs::File;

  pub(crate) fn get_buffered_iter(file: File) -> BufferedTokenIter {
    let token_iter = TokenIter::from(file);
    BufferedTokenIter::from(token_iter)
  }
}
