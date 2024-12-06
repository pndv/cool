use crate::model::class::ParseClass;

#[derive(PartialEq, Debug, Clone)]
pub struct ParseProgram {
    pub classes: Vec<ParseClass>,
}

impl Default for ParseProgram {
    fn default() -> Self {
        Self::new()
    }
}

impl ParseProgram {
    #[must_use]
    pub(crate) fn new() -> Self {
        let classes: Vec<ParseClass> = Vec::new();
        ParseProgram { classes }
    }

    pub(crate) fn add_class(&mut self, class: ParseClass) {
        self.classes.push(class);
    }

    #[must_use]
    pub fn classes(&self) -> &Vec<ParseClass> {
        &self.classes
    }
}
