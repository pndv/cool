use crate::model::feature::Feature;
use crate::model::Type;
use std::borrow::Cow;

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Class {
  name: Type,
  parent_type: Option<Type>, // if no parent is given, then 'Object' is the parent of all classes
  features: Option<Vec<Feature>>,
}

const OBJECT: Class = Class {
  name: Type(Cow::Borrowed("Object")),
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
