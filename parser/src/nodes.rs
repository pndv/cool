use crate::tokens::Token;
use std::borrow::Cow;

// ID: TYPE [[ <- Expression ]]
pub type Id = (Cow<'static, str>, u32, u32);

pub type Type = (Cow<'static, str>, u32, u32);

impl From<Token> for Type { // Type and Symbol have same implementation of From
  fn from(value: Token) -> Self {
    match value {
      Token::Ident { value, line_num, line_pos } => (Cow::Owned(value), line_num, line_pos),
      _ => panic!("Unexpected token {:?}", value),
    }
  }
}

#[cfg(test)]
mod test {
  use crate::expressions::Expression;

  #[test]
  fn test_get_type() {
    assert_eq!(Expression::Int { value: 10, line_num: 5, line_pos: 10 }.get_type(), "Int");
  }
}
