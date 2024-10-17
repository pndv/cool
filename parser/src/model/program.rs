use crate::model::class::Class;

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Program {
    classes: Vec<Class>,
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Program {
    #[must_use]
    pub fn new() -> Self {
        let classes: Vec<Class> = Vec::new();
        Program { classes }
    }

    pub(crate) fn add_class(&mut self, class: Class) {
        self.classes.push(class);
    }

    #[must_use]
    pub fn classes(&self) -> &Vec<Class> {
        &self.classes
    }
}
