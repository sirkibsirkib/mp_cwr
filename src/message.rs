use crate::{MsgStore, Program, SignedMessage};

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

    pub(crate) fn send(&mut self, sm: SignedMessage) -> Result<(), ()> {
        let SignedMessage { message, signature } = sm;
        if let Some(m2) = self.map.get(&signature) {
            if m2 != &message {
                return Err(());
            }
        }
        Ok(())
    }
}
