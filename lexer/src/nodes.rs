pub type Symbol = String;
pub type Type = String;
pub type CaseBranch = (String, Symbol, Box<Expression>);

pub struct Program {
    classes: Vec<Class>,
}

impl Program {
    pub fn new() -> Self {
        let classes: Vec<Class> = Vec::new();
        Program { classes }
    }

    pub fn add_class(&mut self, class: Class) {
        self.classes.push(class);
    }
}


pub struct Class {
    type_name: Type,
    parent_type: Option<Type>, // if no parent is given, then 'Object' is the parent of all classes
    features: Option<Vec<Feature>>,
}

impl Class {
    const OBJECT: Class = Class {
        type_name: "Object".to_string(),
        parent_type: None,
        features: None
    };

    pub fn new(type_name: String, parent_type: Option<Type>, features: Option<Vec<Feature>>) -> Self {
        let parent: Type;
        if parent_type.is_some() {
            parent = parent_type.unwrap();
        } else {
            parent = Self::OBJECT.type_name.clone();
        }

        Class {
            type_name,
            parent_type: Some(parent),
            features,
        }
    }
}

pub struct Feature {
    pub ident: Symbol,
    pub formals: Option<Vec<Formal>>,
    pub f_type: Type,
    pub expr: Box<Expression>
}

pub struct Formal {
    pub ident: Symbol,
    pub f_type: Type,
}

pub enum Expression {
    NoExpr,

    Assign { name: Symbol, expr: Box<Expression>},

    StaticDispatch {expr: Box<Expression>, type_name: Symbol, name: Symbol, actual: Box<Expression>},
    Dispatch {expr: Box<Expression>, name: Symbol, actual: Box<Expression>},

    Conditional {predicate: Box<Expression>, then_exp: Box<Expression>, else_exp: Box<Expression>},

    Loop {predicate: Box<Expression>, body: Symbol, actual: Box<Expression>},

    Case {switch_expression: Box<Expression>, branches: Vec<CaseBranch>},

    Block {body: Box<Expression>},

    Let {identifier: Symbol, type_declaration: Symbol,  init: Box<Expression>, body: Box<Expression>},

    Plus {left: Box<Expression>, right: Box<Expression>},
    Minus {left: Box<Expression>, right: Box<Expression>},
    Multiply {left: Box<Expression>, right: Box<Expression>},
    Divide {left: Box<Expression>, right: Box<Expression>},

    Negate {expr: Box<Expression>},

    LessThan {left: Box<Expression>, right: Box<Expression>},
    Equal {left: Box<Expression>, right: Box<Expression>},
    LessThanOrEqual {left: Box<Expression>, right: Box<Expression>},

    Comp {expr: Box<Expression>},

    Int {value: i32},
    Bool {value: bool},
    String {value: String},

    New {type_name: Symbol},
    IsVoid{expr: Box<Expression>},

    Object {name: Symbol},
}
