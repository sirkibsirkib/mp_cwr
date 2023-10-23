use seaso::{dynamics::Atom, util::VecSet, Program};
use std::collections::HashMap;

mod crypto;
mod message;
mod program;
mod state;

struct PrivKey(u64);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct PubKey(u64);

type Agent = PubKey;
type Epoch = usize;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Ord, PartialOrd)]
struct Signature(u64);

#[derive(Debug, Hash, Eq, PartialEq)]
struct Message {
    program: Program,
    include: VecSet<Signature>,
    exclude: VecSet<Signature>,
}

// these are sent on the wire
struct SignedMessage {
    message: Message,
    signature: Signature,
}

#[derive(Clone)]
enum Agreement {
    AnythingGoes,
}

#[derive(Default)]
struct MsgStore {
    map: HashMap<Signature, Message>,
}

#[derive(Default)]
struct AgentState {
    // inbox: Inbox,
    epoch_states: Vec<EpochState>,
}
struct EpochState {
    agreement: Agreement,
    event_to_justification: HashMap<Atom, MsgStore>,
}

struct State {
    sent: MsgStore,
    agent_states: HashMap<PubKey, AgentState>,
}

trait ErrOr<O> {
    type Out;
    fn err_or(self, o: O) -> Self::Out;
}

////////////////////////////////////////////////////////////////////////////////

impl<E, O> ErrOr<O> for Option<E> {
    type Out = Result<O, E>;
    fn err_or(self, o: O) -> Result<O, E> {
        match self {
            Some(e) => Err(e),
            None => Ok(o),
        }
    }
}

fn main() {
    let mut state = State::new([PubKey(0), PubKey(1)]);
    state.consensus(0, Agreement::AnythingGoes).unwrap();
}
