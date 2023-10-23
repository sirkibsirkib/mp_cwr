use crate::{Message, PrivKey, PubKey, Signature, SignedMessage};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

impl PrivKey {
    pub fn sign(message: Message) -> SignedMessage {
        let mut hasher = DefaultHasher::new();
        message.hash(&mut hasher);
        SignedMessage { message, signature: Signature(hasher.finish()) }
    }
}

impl PubKey {
    pub fn verify(sm: &SignedMessage) -> bool {
        let mut hasher = DefaultHasher::new();
        sm.message.hash(&mut hasher);
        sm.signature == Signature(hasher.finish())
    }
}
