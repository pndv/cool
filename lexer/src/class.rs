use std::borrow::Cow;
use crate::feature;
use crate::feature::Feature;
use crate::nodes::Type;
use crate::tokens::{match_required_token, peek_token_eq, peek_token_neq, FilteredTokensIterator, Token, CLASS_TYPE, CLOSE_CURL_TYPE, IDENT_TYPE, INHERITS_TYPE, OPEN_CURL_TYPE, SEMI_COLON_TYPE};

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Class {
  class_type: Type,
  parent_type: Option<Type>, // if no parent is given, then 'Object' is the parent of all classes
  features: Option<Vec<Feature>>,
}

const OBJECT: Class = Class {
  class_type: (Cow::Borrowed("Object"), 0, 0),
  parent_type: None,
  features: None,
};

impl Class {
  pub(crate) fn new(class_type: Type, parent_type: Option<Type>, features: Option<Vec<Feature>>) -> Self {
    let parent: Type = if parent_type.is_some() {
      parent_type.unwrap()
    } else {
      OBJECT.class_type.clone()
    };

    Class {
      class_type,
      parent_type: Some(parent),
      features,
    }
  }

  pub fn add_feature(&mut self, feature: Feature) {
    if self.features.is_none() {
      self.features = Some(Vec::new());
    }

    if let Some(ref mut features) = self.features {
      features.push(feature);
    }
  }
}

pub(crate) fn gen_class(token_iter: &mut FilteredTokensIterator, read_till_tokens: &[Token]) -> Class {
  match_required_token(token_iter.next(), CLASS_TYPE);

  let mut token = match_required_token(token_iter.next(), IDENT_TYPE);
  let class_type: Type = Type::from(token);

  let mut parent_type: Option<Type> = None;
  if peek_token_eq(token_iter, &INHERITS_TYPE) {
    match_required_token(token_iter.next(), INHERITS_TYPE);

    token = match_required_token(token_iter.next(), IDENT_TYPE);
    let inherits_from: Type = Type::from(token);
    parent_type = Some(inherits_from);
  }

  match_required_token(token_iter.next(), OPEN_CURL_TYPE);

  let mut features: Option<Vec<Feature>> = None;
  if peek_token_neq(token_iter, &CLOSE_CURL_TYPE) {
    features = feature::gen_features(token_iter, read_till_tokens);
  }

  match_required_token(token_iter.next(), CLOSE_CURL_TYPE);

  Class::new(class_type, parent_type, features)
}

#[cfg(test)]
mod test_class {
  use crate::class::{gen_class, Class};
  use crate::feature::gen_feature;
  use crate::tokens::{get_filtered_token_iter, FilteredTokensIterator};

  #[test]
  pub fn test_class() {
    let file_path = "test_resources/classes/class.1.cl_partial";
    let mut token_iter: FilteredTokensIterator = get_filtered_token_iter(file_path);
    let class: Class = gen_class(&mut token_iter, &[]);

  }
}