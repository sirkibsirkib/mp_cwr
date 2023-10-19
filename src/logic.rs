use crate::{Atom, Denotation, Program, ProgramIllFormity};

impl Program {
    pub fn complete_annotations(&mut self) {}

    pub fn denotation(&self) -> Result<Denotation, ProgramIllFormity> {
        todo!()
    }
}

impl Denotation {
    fn value_of(&self, atom: &Atom) -> Option<bool> {
        if self.truths.contains(atom) {
            Some(true)
        } else if self.unknowns.contains(atom) {
            None
        } else {
            Some(false)
        }
    }
}
