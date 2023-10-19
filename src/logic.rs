use crate::Set;
use crate::{Atom, Denotation, Domain, Program, ProgramIllFormity, Rule, Statement, Variable};
use std::collections::HashMap;

trait Contains<T> {
    fn visit(&self, visitor: &mut impl FnMut(&T));
    fn visit_mut(&mut self, visitor: &mut impl FnMut(&mut T));
}

impl Contains<Atom> for Atom {
    fn visit(&self, visitor: &mut impl FnMut(&Atom)) {
        visitor(self);
        if let Self::Construct { args, .. } = self {
            for arg in args {
                arg.visit(visitor)
            }
        }
    }
    fn visit_mut(&mut self, visitor: &mut impl FnMut(&mut Atom)) {
        visitor(self);
        if let Self::Construct { args, .. } = self {
            for arg in args {
                arg.visit_mut(visitor)
            }
        }
    }
}

impl Contains<Atom> for Vec<Atom> {
    fn visit(&self, visitor: &mut impl FnMut(&Atom)) {
        for atom in self {
            atom.visit(visitor)
        }
    }
    fn visit_mut(&mut self, visitor: &mut impl FnMut(&mut Atom)) {
        for atom in self {
            atom.visit_mut(visitor)
        }
    }
}

struct Definitions {
    map: HashMap<Domain, Vec<Domain>>,
}

impl Rule {
    pub fn range_restricted(&self) -> bool {
        let mut f = |atom: &Atom, set: &mut Set<&Variable>| {
            if let Atom::Variable { var } = atom {
                set.insert(&var);
            }
        };
        let mut c = Set::default();
        self.consequents.vec.visit(&mut |atom| f(atom, &mut c));
        let mut p = Set::default();
        self.consequents.vec.visit(&mut |atom| f(atom, &mut p));
        c.is_subset(&p)
    }
}

impl Program {
    fn rules(&self) -> impl Iterator<Item = &Rule> {
        self.statements.iter().filter_map(|statement| {
            if let Statement::Rule(r) = statement {
                Some(r)
            } else {
                None
            }
        })
    }
    fn range_restricted(&self) -> bool {
        self.rules().all(Rule::range_restricted)
    }

    pub fn denotation(mut self) -> Result<Denotation, ProgramIllFormity> {
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
