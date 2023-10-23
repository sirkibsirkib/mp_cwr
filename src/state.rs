use crate::Agreement;
use crate::Epoch;
use crate::EpochState;
use crate::ErrOr;
use crate::PubKey;
use crate::State;

impl State {
    pub fn new(agents: impl IntoIterator<Item = PubKey>) -> Self {
        State {
            sent: Default::default(),
            agent_states: agents.into_iter().map(|agent| (agent, Default::default())).collect(),
        }
    }

    pub fn consensus(&mut self, epoch: Epoch, agreement: Agreement) -> Result<(), PubKey> {
        self.agent_states
            .iter()
            .filter_map(|(agent, agent_state)| {
                if agent_state.epoch_states.len() != epoch {
                    Some(agent.clone())
                } else {
                    None
                }
            })
            .next()
            .err_or(())?;
        for agent_state in self.agent_states.values_mut() {
            agent_state.epoch_states.push(EpochState {
                agreement: agreement.clone(),
                event_to_justification: Default::default(),
            })
        }
        Ok(())
    }
}
