use crate::models::features::{AttributeNode, FeatureNode, MethodNode};
use crate::models::Node;
use parser::model::class::{ParseClass, BOOL_CLASS_NAME, INT_CLASS_NAME, IO_CLASS_NAME, OBJECT_CLASS_NAME, STR_CLASS_NAME};
use parser::model::feature::{Attribute, Method, ParseFeature};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub const NO_INHERIT: [&str; 3] = [INT_CLASS_NAME, STR_CLASS_NAME, BOOL_CLASS_NAME];
pub const PRIMITIVE_TYPES: [&str; 5] = [INT_CLASS_NAME, STR_CLASS_NAME, BOOL_CLASS_NAME, IO_CLASS_NAME, OBJECT_CLASS_NAME];

#[derive(Debug, PartialEq, Clone)]
pub struct ClassNode {
  pub name: Cow<'static, str>,
  pub parent: Cow<'static, str>,
  pub line_num: u32,
  pub line_pos: u32,
  pub(crate) children: Vec<Cow<'static, str>>,
  pub(crate) class_symbol_map: HashMap<String, String>,
  pub(crate) features: Vec<FeatureNode>,
  pub(crate) attributes: Vec<AttributeNode>,
  pub(crate) methods: Vec<MethodNode>,
  parent: 
}

impl Node for ClassNode {}

impl ClassNode {
  pub(crate) fn add_child(&mut self, child: Cow<'static, str>) {
    self.children.push(child);
  }
  fn get_children(&self) -> Vec<Cow<'static, str>> { self.children.clone() }

  pub fn get_base_object() -> ClassNode {
    let mut base_object = BASE_NODE_OBJECT.clone();
    for base_child in BASE_CHILDREN {
      base_object.add_child(base_child)
    }

    base_object
  }

  pub fn add_features(&mut self, features: &Vec<ParseFeature>) -> Result<Vec<FeatureNode>, String> {
    let mut feature_nodes = Vec::new();
    let mut errors = String::new();
    for feature in features {
      let result = match feature {
        ParseFeature::Attribute { attribute } => {
          match self.add_attribute(attribute) {
            Ok(attribute_node) => Ok(FeatureNode::Attribute(attribute_node)),
            Err(error) => Err(error)
          }
        }
        ParseFeature::Method { method } => {
          match self.add_method(method) {
            Ok(method_node) => Ok(FeatureNode::Method(method_node)),
            Err(error) => Err(error)
          }
        }
      };

      match result {
        Ok(decorated_feature) => feature_nodes.push(decorated_feature),
        Err(error) => {
          errors.push_str(&error);
          errors.push('\n');
        }
      }
    }

    if errors.is_empty() {
      Ok(feature_nodes)
    } else {
      Err(errors)
    }
  }

  pub fn add_attribute(&mut self, attribute: &Attribute) -> Result<AttributeNode, String> {
    match attribute {
      Attribute { name, return_type, expr } => {
        let attr_name = name.get_name();
        if self.class_symbol_map.contains_key(&attr_name) {
          return Err(format!("Attribute {} already exists in the class {}", attr_name, self.name));
        }

        let attr_type = return_type.get_name();
        self.class_symbol_map.insert(attr_name.clone(), attr_type.clone());

        // Check return_type exists, call get_symbol

        match expr {
          None => Ok(AttributeNode { name: Cow::from(attr_name), attribute_type: Cow::from(attr_type), exp: None }),
          Some(expression) => { todo!("work on expression") }
        }
      }
    }
  }

  pub fn add_method(&mut self, method: &Method) -> Result<MethodNode, String> {
    todo!()
  }
}

impl From<ParseClass> for ClassNode {
  fn from(value: ParseClass) -> Self { Self::from(&value) }
}

impl From<&ParseClass> for ClassNode {
  fn from(value: &ParseClass) -> Self {
    let ParseClass { name, parent_type, .. } = value;
    let class_name = name.get_name();
    let parent = parent_type.get_name();

    ClassNode {
      name: Cow::from(class_name),
      parent: Cow::from(parent),
      line_num: value.line_num,
      line_pos: value.line_pos,
      children: Default::default(),
      class_symbol_map: Default::default(),
      features: Default::default(),
      attributes: Default::default(),
      methods: Default::default(),
    }
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

const BASE_CHILDREN: [Cow<'static, str>; 4] = [Cow::Borrowed(IO_CLASS_NAME),
  Cow::Borrowed(BOOL_CLASS_NAME),
  Cow::Borrowed(INT_CLASS_NAME),
  Cow::Borrowed(STR_CLASS_NAME)];

const BASE_NODE_OBJECT: ClassNode = ClassNode {
  name: Cow::Borrowed(OBJECT_CLASS_NAME),
  parent: Cow::Borrowed(""),
  line_num: 0,
  line_pos: 0,
  children: Default::default(),
  class_symbol_map: Default::default(),
  features: Default::default(),
  attributes: Default::default(),
  methods: Default::default(),
};

pub const BASE_NODE_IO: ClassNode = ClassNode {
  name: Cow::Borrowed(IO_CLASS_NAME),
  parent: Cow::Borrowed(OBJECT_CLASS_NAME),
  line_num: 0,
  line_pos: 0,
  children: Default::default(),
  class_symbol_map: Default::default(),
  features: Default::default(),
  attributes: Default::default(),
  methods: Default::default(),
};

pub const BASE_NODE_INT: ClassNode = ClassNode {
  name: Cow::Borrowed(INT_CLASS_NAME),
  parent: Cow::Borrowed(OBJECT_CLASS_NAME),
  line_num: 0,
  line_pos: 0,
  children: Default::default(),
  class_symbol_map: Default::default(),
  features: Default::default(),
  attributes: Default::default(),
  methods: Default::default(),
};

pub const BASE_NODE_STR: ClassNode = ClassNode {
  name: Cow::Borrowed(STR_CLASS_NAME),
  parent: Cow::Borrowed(OBJECT_CLASS_NAME),
  line_num: 0,
  line_pos: 0,
  children: Default::default(),
  class_symbol_map: Default::default(),
  features: Default::default(),
  attributes: Default::default(),
  methods: Default::default(),
};

pub const BASE_NODE_BOOL: ClassNode = ClassNode {
  name: Cow::Borrowed(BOOL_CLASS_NAME),
  parent: Cow::Borrowed(OBJECT_CLASS_NAME),
  line_num: 0,
  line_pos: 0,
  children: Default::default(),
  class_symbol_map: Default::default(),
  features: Default::default(),
  attributes: Default::default(),
  methods: Default::default(),
};
