use crate::generators::expressions::gen_expression;
use crate::generators::formal::gen_formals;
use crate::model::feature::{Attribute, Method, ParseFeature};
use crate::model::formal::Formal;
use crate::model::{Ident, Type};
use lexer::iter::token::{BaseTokenIter, BufferedTokenIter};
use lexer::model::constants::{
  ASSIGN_TYPE, CLOSE_CURL_TYPE, CLOSE_PAREN_TYPE, COLON_TYPE, IDENT_TYPE, OPEN_CURL_TYPE,
  OPEN_PAREN_TYPE, SEMI_COLON_TYPE,
};
use lexer::model::token::Token;

/// Features :-> {{ features; }}*
pub(super) fn gen_features(iter: &mut BufferedTokenIter) -> Result<Option<Vec<ParseFeature>>, String> {
  let mut features: Vec<ParseFeature> = Vec::new();

  // `{` seen in calling method => read till closing `}` encountered for `class`
  while iter.has_next() {
    let feature: ParseFeature = gen_feature(iter, &SEMI_COLON_TYPE)?;

    // Feature must terminate with a semicolon
    iter.consume_required(&SEMI_COLON_TYPE)?;

    features.push(feature);
  }

  if features.is_empty() {
    Ok(None)
  } else {
    Ok(Some(features))
  }
}

fn gen_feature(iter: &mut BufferedTokenIter, read_till_token: &Token) -> Result<ParseFeature, String> {
  //Feature starts with ID
  let Token::Ident { value, .. } = iter.get_required(&IDENT_TYPE)? else {
    unreachable!()
  };
  let ident_name = Ident::from(value);

  let feature: ParseFeature = match iter.peek() {
    Some(peeked_token) if *peeked_token == COLON_TYPE => {
      let attribute = gen_attribute_feature(ident_name, iter, read_till_token)?;
      ParseFeature::Attribute { attribute }
    }

    Some(peeked_token) if *peeked_token == OPEN_PAREN_TYPE => {
      gen_method_feature(ident_name, iter)?
    }

    Some(t) => return Err(format!("Incorrect token {t}")),

    None => return Err("Unexpected EOF".to_string()),
  };

  Ok(feature)
}

fn gen_method_feature(ident_name: Ident, iter: &mut BufferedTokenIter) -> Result<ParseFeature, String> {
  iter.consume_required(&OPEN_PAREN_TYPE)?;

  let mut formals: Option<Vec<Formal>> = None;

  // `(` seen in calling method => If the next token is not `)`, read formals list
  if iter.has_next() && !iter.peek_eq(&CLOSE_PAREN_TYPE) {
    let vec_formals = gen_formals(iter)?;
    formals = Some(vec_formals);
  }

  iter.consume_required(&CLOSE_PAREN_TYPE)?;
  iter.consume_required(&COLON_TYPE)?;

  let Token::Ident { value, .. } = iter.get_required(&IDENT_TYPE)? else {
    unreachable!()
  };
  let method_return_type = Type::from(value);

  iter.consume_required(&OPEN_CURL_TYPE)?;

  let method_expr = gen_expression(iter, &CLOSE_CURL_TYPE)?;

  iter.consume_required(&CLOSE_CURL_TYPE)?;

  let method = Method { name: ident_name, formals, return_type: method_return_type, expr: method_expr };
  let feature = ParseFeature::Method { method };

  Ok(feature)
}

fn gen_attribute_feature(
  ident_name: Ident,
  iter: &mut BufferedTokenIter,
  read_till_tokens: &Token,
) -> Result<Attribute, String> {
  iter.consume_required(&COLON_TYPE)?;

  let Token::Ident { value, .. } = iter.get_required(&IDENT_TYPE)? else {
    unreachable!()
  };
  let method_return_type = Type::from(value);

  let attribute = if iter.peek_eq(&ASSIGN_TYPE) {
    iter.consume_required(&ASSIGN_TYPE)?;

    let method_expr = gen_expression(iter, read_till_tokens)?;

    Attribute::from((ident_name, method_return_type, Some(method_expr)))
  } else {
    Attribute::from((ident_name, method_return_type, None))
  };

  Ok(attribute)
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::model::feature::Method;
  use crate::test::get_buffered_iter;
  use std::fs::File;

  #[test]
  fn test_feature_method() {
    let file = File::open("../test_resources/features/feature.method_form.cl_partial")
        .expect("file not found");
    let mut token_iter: BufferedTokenIter = get_buffered_iter(file);
    let feature: ParseFeature =
        gen_feature(&mut token_iter, &Token::EOF).expect("feature not generated");

    match feature {
      ParseFeature::Attribute { .. } => {}
      ParseFeature::Method { method } => {
        let Method { name: feature_name, formals, return_type, expr } = method;
        let Ident(name, ..) = feature_name;
        assert_eq!(name, "method2");

        assert!(formals.is_some());
        let Some(mut formal_list) = formals else {
          panic!("Formal list should not be empty")
        };
        assert_eq!(formal_list.len(), 2);

        let Formal {
          formal_name,
          formal_type,
        } = formal_list.pop().unwrap();
        let Ident(f_name, ..) = formal_name;
        let Type(f_type, ..) = formal_type;
        assert_eq!(f_name, "num2");
        assert_eq!(f_type, "Int");

        let Formal {
          formal_name,
          formal_type,
        } = formal_list.pop().unwrap();
        let Ident(f_name, ..) = formal_name;
        let Type(f_type, ..) = formal_type;
        assert_eq!(f_name, "num1");
        assert_eq!(f_type, "Int");

        let Type(formal_return_type, ..) = return_type;
        assert_eq!(formal_return_type, "B");

        assert_eq!(expr.get_type(), "Let");
      }
    }
  }

  #[test]
  fn test_feature_method_self_type() {
    let file =
        File::open("../test_resources/features/feature.method_form_self_type.cl_partial")
            .expect("file not found");
    let mut token_iter: BufferedTokenIter = get_buffered_iter(file);
    let feature: ParseFeature =
        gen_feature(&mut token_iter, &Token::EOF).expect("feature not generated");
    assert!(matches!(feature, ParseFeature::Method { .. }));

    match feature {
      ParseFeature::Attribute {..} => assert!(false),
      ParseFeature::Method { method } => {
        let Method { name: feature_name, formals, return_type, expr } = method;

        let Ident(name, ..) = feature_name;
        assert_eq!(name, "main");

        assert!(formals.is_none());

        let Type(formal_return_type, ..) = return_type;
        assert_eq!(formal_return_type, "SELF_TYPE");
        assert_eq!(expr.get_type(), "Block");
        println!("{expr}");
      }
    }
  }

  #[test]
  fn test_feature_attribute_without_expr() {
    let file = File::open("../test_resources/features/feature.attribute_no_expr.cl_partial")
        .expect("file not found");
    let mut token_iter: BufferedTokenIter = get_buffered_iter(file);
    let feature: ParseFeature =
        gen_feature(&mut token_iter, &Token::EOF).expect("Error parsing feature");

    match feature {
      ParseFeature::Attribute { attribute } => {
        let Attribute { name: feature_name, return_type, expr } = attribute;
        let Ident(name, ..) = feature_name;
        assert_eq!(name, "population_map");

        let Type(formal_return_type, ..) = return_type;
        assert_eq!(formal_return_type, "String");

        assert!(expr.is_none());
      }
      ParseFeature::Method { .. } => assert!(false),
    }
  }

  #[test]
  fn test_feature_list() {
    let file = File::open("../test_resources/features/feature.list.cl_partial")
        .expect("file not found");
    let mut token_iter: BufferedTokenIter = get_buffered_iter(file);

    let features = gen_features(&mut token_iter).expect("Error parsing feature");
    assert!(features.is_some());

    let Some(feature_list) = features else {
      panic!("FeatureList should not be empty")
    };

    assert_eq!(feature_list.len(), 5);
  }

  #[test]
  fn test_feature_attribute_with_expr() {
    let file = File::open("../test_resources/features/feature.attribute_with_expr.cl_partial")
        .expect("file not found");
    let mut token_iter: BufferedTokenIter = get_buffered_iter(file);
    let feature: ParseFeature =
        gen_feature(&mut token_iter, &Token::EOF).expect("Error parsing feature");

    match feature {
      ParseFeature::Attribute { attribute } => {
        let Attribute { name: feature_name, return_type, expr } = attribute;

        let Ident(name, ..) = feature_name;
        assert_eq!(name, "vertices");

        let Type(formal_return_type, ..) = return_type;
        assert_eq!(formal_return_type, "VList");

        assert!(expr.is_some());
        let Some(feature_expr) = expr else { unreachable!();};
        assert_eq!(feature_expr.get_type(), "New");
      },
      ParseFeature::Method { .. } => assert!(false),
    }
  }
}
