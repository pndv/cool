use crate::model::feature::ParseFeature;
use crate::model::Type;
use std::borrow::Cow;

#[derive(PartialEq, Debug, Clone)]
pub struct ParseClass {
    pub name: Type,
    pub parent_type: Type, // if no parent is given, then 'Object' is the parent of all classes
    pub features: Option<Vec<ParseFeature>>,
    pub line_num: u32,
    pub line_pos: u32,
}

pub const OBJECT_CLASS_NAME: &str = "Object";
pub const IO_CLASS_NAME: &str = "IO";
pub const INT_CLASS_NAME: &str = "Int";
pub const STR_CLASS_NAME: &str = "String";
pub const BOOL_CLASS_NAME: &str = "Bool";
pub const VOID_CLASS_NAME: &str = "Void";

pub const OBJECT: ParseClass = ParseClass {
    name: Type(Cow::Borrowed(OBJECT_CLASS_NAME)),
    parent_type: Type(Cow::Borrowed("BASE_OBJECT")),
    features: None,
    line_num: 0,
    line_pos: 0,
};

impl ParseClass {
    pub fn get_name(&self) -> String {
        self.name.get_name()
    }

    pub(crate) fn new(
        class_type: Type,
        parent_type: Option<Type>,
        features: Option<Vec<ParseFeature>>,
        line_num: u32,
        line_pos: u32,
    ) -> Self {
        let parent: Type = if parent_type.is_some() {
            parent_type.unwrap()
        } else {
            OBJECT.name.clone()
        };

        ParseClass {
            name: class_type,
            parent_type: parent,
            features,
            line_num,
            line_pos,
        }
    }
}
