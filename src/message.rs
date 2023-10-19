use crate::{MsgStore, Program};

impl MsgStore {
    pub(crate) fn program(&self) -> Program {
        self.map.values().map(|msg| msg.program.clone()).fold(Program::default(), Program::composed)
    }

    pub(crate) fn internally_consisent(&self) -> bool {
        self.map.values().all(|msg| {
            msg.include.iter().all(|sig| self.map.contains_key(sig))
                && !msg.exclude.iter().any(|sig| self.map.contains_key(sig))
        })
    }
}
