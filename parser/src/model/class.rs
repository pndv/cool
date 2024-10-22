use crate::model::feature::Feature;
use crate::model::Type;
use std::borrow::Cow;

#[derive(PartialEq, Debug, Clone)]
pub struct Class {
    pub name: Type,
    pub parent_type: Type, // if no parent is given, then 'Object' is the parent of all classes
    pub features: Option<Vec<Feature>>,
    pub line_num: u32,
    pub line_pos: u32,
}

pub const OBJECT_CLASS_NAME: &str = "Object";
pub const IO_CLASS_NAME: &str = "IO";
pub const INT_CLASS_NAME: &str = "Int";
pub const STR_CLASS_NAME: &str = "String";
pub const BOOL_CLASS_NAME: &str = "Bool";

pub const OBJECT: Class = Class {
    name: Type(Cow::Borrowed(OBJECT_CLASS_NAME)),
    parent_type: Type(Cow::Borrowed("BASE_OBJECT")),
    features: None,
    line_num: 0,
    line_pos: 0,
};

impl Class {
    pub(crate) fn new(
        class_type: Type,
        parent_type: Option<Type>,
        features: Option<Vec<Feature>>,
        line_num: u32,
        line_pos: u32,
    ) -> Self {
        let parent: Type = if parent_type.is_some() {
            parent_type.unwrap()
        } else {
            OBJECT.name.clone()
        };

        Class {
            name: class_type,
            parent_type: parent,
            features,
            line_num,
            line_pos,
        }
    }
}
