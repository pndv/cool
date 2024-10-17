use crate::feature;
use crate::feature::Feature;
use crate::nodes::Type;
use crate::tokens::{consume_required, gen_iter_till_token_or_end, match_required_token, peek_not_eq_or_eof, peek_token_eq, FilteredTokensIterator, Token, CLASS_TYPE, CLOSE_CURL_TYPE, IDENT_TYPE, INHERITS_TYPE, OPEN_CURL_TYPE};
use feature::gen_features;
use std::borrow::Cow;

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Class {
  name: Type,
  parent_type: Option<Type>, // if no parent is given, then 'Object' is the parent of all classes
  features: Option<Vec<Feature>>,
}

const OBJECT: Class = Class {
  name: (Cow::Borrowed("Object"), 0, 0),
  parent_type: None,
  features: None,
};

impl Class {
  pub(crate) fn new(class_type: Type, parent_type: Option<Type>, features: Option<Vec<Feature>>) -> Self {
    let parent: Type = if parent_type.is_some() {
      parent_type.unwrap()
    } else {
      OBJECT.name.clone()
    };

    Class {
      name: class_type,
      parent_type: Some(parent),
      features,
    }
  }
}

pub(crate) fn gen_class(token_iter: &mut FilteredTokensIterator) -> Class {
  consume_required(token_iter, CLASS_TYPE);

  let mut token = match_required_token(token_iter.next(), IDENT_TYPE);
  let class_type: Type = Type::from(token);

  let mut parent_type: Option<Type> = None;
  if peek_token_eq(token_iter, &INHERITS_TYPE) {
    consume_required(token_iter, INHERITS_TYPE);

    token = match_required_token(token_iter.next(), IDENT_TYPE);
    let inherits_from: Type = Type::from(token);
    parent_type = Some(inherits_from);
  }

  consume_required(token_iter, OPEN_CURL_TYPE);

  let mut features: Option<Vec<Feature>> = None;
  if peek_not_eq_or_eof(token_iter, &CLOSE_CURL_TYPE) {
    let mut feature_token_iter: FilteredTokensIterator = gen_iter_till_token_or_end(token_iter, &CLOSE_CURL_TYPE);

    features = gen_features(&mut feature_token_iter, &Token::EOF);
  }

  consume_required(token_iter, CLOSE_CURL_TYPE);

  Class::new(class_type, parent_type, features)
}

#[cfg(test)]
mod test_class {
  use crate::class::{gen_class, Class};
  use crate::tokens::{get_filtered_token_iter, FilteredTokensIterator};

  #[test]
  pub fn test_class_single_feature() {
    let file_path = "test_resources/classes/class.single_feature.cl_partial";
    let mut token_iter: FilteredTokensIterator = get_filtered_token_iter(file_path);
    let class: Class = gen_class(&mut token_iter);
    println!("{:?}", class);

    assert!(class.features.is_some());

    let features = class.features.unwrap();
    assert_eq!(features.len(), 1);
  }

  #[test]
  pub fn test_class_multi_feature() {
    let file_path = "test_resources/classes/class.multi_feature.cl_partial";
    let mut token_iter: FilteredTokensIterator = get_filtered_token_iter(file_path);
    let class: Class = gen_class(&mut token_iter);
    println!("{:?}", class);
    assert!(class.features.is_some());

    let features = class.features.unwrap();
    assert_eq!(features.len(), 3);
  }
}
