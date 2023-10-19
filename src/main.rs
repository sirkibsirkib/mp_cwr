use seaso::{dynamics::Atom, Program};
use std::collections::HashMap;

mod message;
mod set;
mod state;

struct PrivKey(u128);
struct PubKey(u128);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Ord, PartialOrd)]
struct Signature(u128);

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Domain(String);

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Variable {
    dom: Domain,
    suffix: u32,
}

type Agent = PubKey;
type Epoch = usize;

#[derive(Debug, Ord, PartialEq, Eq, PartialOrd)]
struct Set<T: Ord> {
    // vecset implementation
    vec: Vec<T>,
}

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

struct AgentState {
    inbox: Inbox,
    epoch_states: Vec<EpochState>,
}
struct EpochState {
    agreement: Agreement,
    events: HashMap<Atom, HashMap<Signature, Message>>,
}

struct Inbox {
    signature_to_message: HashMap<Signature, Message>,
}

fn main() {
    println!("Hello, world!");
}
