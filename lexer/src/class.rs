use std::borrow::Cow;
use feature::gen_features;
use crate::feature;
use crate::feature::Feature;
use crate::nodes::Type;
use crate::tokens::{consume_required, is_eof, match_required_token, peek_token_eq, peek_token_neq_or_eof, FilteredTokensIterator, Token, CLASS_TYPE, CLOSE_CURL_TYPE, IDENT_TYPE, INHERITS_TYPE, OPEN_CURL_TYPE, SEMI_COLON_TYPE};

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

pub(crate) fn gen_class(token_iter: &mut FilteredTokensIterator, read_till_tokens: &Token) -> Class {
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
  if !is_eof(token_iter) && peek_token_neq_or_eof(token_iter, &CLOSE_CURL_TYPE) {
    features = gen_features(token_iter, &SEMI_COLON_TYPE);
    consume_required(token_iter, SEMI_COLON_TYPE); // features in class are terminated by semicolon
  }

  consume_required(token_iter, CLOSE_CURL_TYPE);

  Class::new(class_type, parent_type, features)
}

#[cfg(test)]
mod test_class {
  use crate::class::{gen_class, Class};
  use crate::tokens::{get_filtered_token_iter, FilteredTokensIterator, Token};

  #[test]
  pub fn test_class() {
    let file_path = "test_resources/classes/class.1.cl_partial";
    let mut token_iter: FilteredTokensIterator = get_filtered_token_iter(file_path);
    let class: Class = gen_class(&mut token_iter, &Token::EOF);
    
    println!("{:?}", class);
  }
}