use crate::{Message, Signature};
use std::collections::HashMap;

pub(crate) fn internally_consisent_messages(map: &HashMap<Signature, Message>) -> bool {
    map.values().all(|msg| {
        msg.include.iter().all(|sig| map.contains_key(sig))
            && !msg.exclude.iter().any(|sig| map.contains_key(sig))
    })
}
