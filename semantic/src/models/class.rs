use crate::models::features::FeatureNode;
use crate::models::Node;
use parser::model::class::{Class, BOOL_CLASS_NAME, INT_CLASS_NAME, IO_CLASS_NAME, OBJECT_CLASS_NAME, STR_CLASS_NAME};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};

pub const NO_INHERIT: [&str; 3] = [INT_CLASS_NAME, STR_CLASS_NAME, BOOL_CLASS_NAME];
pub const PRIMITIVE_TYPES: [&str; 5] = [INT_CLASS_NAME, STR_CLASS_NAME, BOOL_CLASS_NAME, IO_CLASS_NAME, OBJECT_CLASS_NAME];

#[derive(Debug, PartialEq, Clone)]
pub struct ClassNode {
  pub name: Cow<'static, str>,
  pub parent: Cow<'static, str>,
  pub(crate) children: Vec<Cow<'static, str>>,
  pub(crate) features: Vec<dyn FeatureNode>,
}

impl Node for ClassNode {}

impl ClassNode {
  pub(crate) fn add_child(&mut self, child: Cow<'static, str>) {
    self.children.push(child);
  }
  fn get_children(&self) -> Vec<Cow<'static, str>> {self.children.clone()}
}

impl From<Class> for ClassNode {
  fn from(value: Class) -> Self {
    let Class { name, parent_type, .. } = value;
    let class_name = name.get_name();
    let parent = parent_type.get_name();
    let children = Vec::new();

    ClassNode { name: Cow::from(class_name), parent: Cow::from(parent), children, features: Vec::new() }
  }
}

impl From<&Class> for ClassNode {
  fn from(value: &Class) -> Self {
    let Class { name, parent_type, .. } = value;
    let class_name = name.get_name();
    let parent = parent_type.get_name();
    let children = Vec::new();

    ClassNode { name: Cow::from(class_name), parent: Cow::from(parent), children, features: Vec::new() }
  }
}

impl Display for ClassNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let children: String = if self.children.len() > 0 {
      self.children.join(",")
    } else {
      " None".to_string()
    };
    write!(f, "[[ {} ]] Inherits from [[ {} ]] with children: [[ {} ]]", self.name, self.parent, children)
  }
}

pub const BASE_NODE_OBJECT: ClassNode = ClassNode {
  name: Cow::Borrowed(OBJECT_CLASS_NAME),
  parent: Cow::Borrowed(""),
  children: vec![Cow::Borrowed(IO_CLASS_NAME), 
                 Cow::Borrowed(BOOL_CLASS_NAME), 
                 Cow::Borrowed(INT_CLASS_NAME), 
                 Cow::Borrowed(STR_CLASS_NAME)],
  features: Vec::new(),
};
pub const BASE_NODE_IO: ClassNode = ClassNode {
  name: Cow::Borrowed(IO_CLASS_NAME),
  parent: Cow::Borrowed(OBJECT_CLASS_NAME),
  children: Vec::new(),
  features: Vec::new(),
};

pub const BASE_NODE_INT: ClassNode = ClassNode {
  name: Cow::Borrowed(INT_CLASS_NAME),
  parent: Cow::Borrowed(OBJECT_CLASS_NAME),
  children: Vec::new(),
  features: Vec::new(),
};

pub const BASE_NODE_STR: ClassNode = ClassNode {
  name: Cow::Borrowed(STR_CLASS_NAME),
  parent: Cow::Borrowed(OBJECT_CLASS_NAME),
  children: Vec::new(),
  features: Vec::new(),
};

pub const BASE_NODE_BOOL: ClassNode = ClassNode {
  name: Cow::Borrowed(BOOL_CLASS_NAME),
  parent: Cow::Borrowed(OBJECT_CLASS_NAME),
  children: Vec::new(),
  features: Vec::new(),
};
