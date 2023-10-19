use seaso::{dynamics::Atom, Program};
use std::collections::HashMap;

mod message;
mod set;
mod state;

struct PrivKey(u128);
struct PubKey(u128);

type Agent = PubKey;
type Epoch = usize;

#[derive(Debug, Ord, PartialEq, Eq, PartialOrd)]
struct Set<T: Ord> {
    // vecset implementation
    vec: Vec<T>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Ord, PartialOrd)]
struct Signature(u128);

struct Message {
    program: Program,
    include: Set<Signature>,
    exclude: Set<Signature>,
}

// these are sent on the wire
struct SignedMessage {
    message: Message,
    signature: Signature,
}

enum Agreement {
    AnythingGoes,
}

struct MsgStore {
    map: HashMap<Signature, Message>,
}

struct AgentState {
    inbox: Inbox,
    epoch_states: Vec<EpochState>,
}
struct EpochState {
    agreement: Agreement,
    event_to_justification: HashMap<Atom, MsgStore>,
}

struct Inbox {
    signature_to_message: MsgStore,
}
