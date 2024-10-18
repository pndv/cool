use crate::model::class::Class;

#[derive(PartialEq, Debug, Clone)]
pub struct Program {
    pub classes: Vec<Class>,
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Program {
    #[must_use]
    pub(crate) fn new() -> Self {
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
