use std::collections::HashMap;

mod logic;
mod message;
mod set;
mod state;

struct PrivKey(u128);
struct PubKey(u128);

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Signature(u128);

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Domain(String);

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Variable(String);

type Agent = PubKey;
type Epoch = usize;

struct Set<T> {
    // vecset implementation
    vec: Vec<T>,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Atom {
    Variable { var: Variable },
    Construct { dom: Domain, args: Vec<Atom> },
}

struct GroundAtom {
    dom: Domain,
    args: Vec<Atom>,
}

enum Statement {
    Decl {
        introduced_and_equated: Vec<Domain>,
    },
    Defn {
        dom: Domain,
        params: Vec<Domain>,
    },
    Rule {
        consequents: Set<Atom>,
        positive: Set<Atom>,
        negative: Set<Atom>,
        var_annotations: HashMap<Variable, Domain>,
    },
}

enum ProgramIllFormity {
    VariableTwoDomains { var: Variable, doms: [Domain; 2] },
    VariableZeroDomains { var: Variable },
    CyclicConstruction { dom: Domain },
}

struct Denotation {
    truths: Set<Atom>,
    unknowns: Set<Atom>,
}

struct Program {
    statements: Set<Statement>,
}

struct Message {
    statements: Set<Statement>,
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
    events: HashMap<GroundAtom, HashMap<Signature, Message>>,
}

struct Inbox {
    signature_to_message: HashMap<Signature, Message>,
}

fn main() {
    println!("Hello, world!");
}
