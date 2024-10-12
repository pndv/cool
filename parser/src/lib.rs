
pub mod program;
pub(crate) mod class;
pub(crate) mod feature;
pub(crate) mod formal;
pub(crate) mod expressions;
mod nodes;

pub fn add(left: u64, right: u64) -> u64 {
  left + right
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
